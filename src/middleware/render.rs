#![allow(dead_code)]
/// # Render engine middlware
/// Currently only `HandlebarsEngine` support
///
/// ## Features
/// * Init Tempalte engine
/// * Adding Template paths
/// * useful additional helpers with strong params checking
/// * helpers logger for critical situations
///
/// ## Helpers
/// * `link` - css link helper
/// * `script` - js  link helper
///
/// ## How to use
/// ```
/// // Add Template renderer and views path
/// let paths = vec!["./views/"];
/// chain.link_after(iron_cms::middleware::template_render(paths));
/// ```
/// it's included paths ./views/**/*
use iron::prelude::*;
use iron::status;
use hbs::{HandlebarsEngine, DirectorySource};
use handlebars::{Handlebars, RenderError, RenderContext, Helper, Context};
use std::error::Error;
use std::collections::BTreeMap;
use rustc_serialize::json::{Json, ToJson};
use hbs::{Template};

pub type BaseDataMap = BTreeMap<String, Json>;
pub type RenderResult = IronResult<Response>;
pub struct Render {
    pub data : BTreeMap<String, Json>
}

impl Render {
    pub fn new<T: ToJson>(name: &str, data: T) -> IronResult<Response> {
        let mut resp = Response::new();
        resp.set_mut(Template::new(name, data)).set_mut(status::Ok);
        Ok(resp)
    }
}

/// Init Template renderer and add Tempaltes path.
/// It invoke to after middleware
pub fn template_render(paths: Vec<&str>) -> HandlebarsEngine {
    // First init Handlebars
    let mut hregistry = Handlebars::new();

    // Add helpers to Handlebars
    hregistry.register_helper("link", Box::new(link_helper));
    hregistry.register_helper("script", Box::new(script_helper));
    hregistry.register_helper("active", Box::new(active_page));

    // Our instance HandlebarsEngine depended of Handlebars
    let mut template = HandlebarsEngine::from(hregistry);

    // Add a directory source, all files with .html suffix
    // will be loaded as template
    for path in paths.iter() {
        template.add(Box::new(DirectorySource::new(path, ".html")));
    }

    // load templates from all registered sources
    if let Err(r) = template.reload() {
        // Paniced cause it critical situation
        panic!("{:?}", r.description());
    }
    template
}

/// Css link Helper
/// usege: `{{#link ["some/url1", "some/url2"]}}{{/link}}`
fn link_helper(_: &Context, h: &Helper, _: &Handlebars, rc: &mut RenderContext) -> Result<(), RenderError> {
    let css_links = try!(h.param(0)
            .and_then(|v| v.value().as_array())
            .ok_or(RenderError::new("|> link_helper - param 1 with array type is required")));
    let mut css = "".to_owned();
    for link in css_links.iter() {
        let link = try!(link
                    .as_string()
                    .ok_or(RenderError::new("|> link_helper - array param with string type is required")));
        css = format!("{}\t<link rel=\"stylesheet\" type=\"text/css\" href=\"{}\">\n", css, link);
    }
    try!(rc.writer.write(css.into_bytes().as_ref()));
    Ok(())
}

/// Js link Helper
/// usege: `{{#script ["some/url1", "some/url2"]}}{{/script}}`
fn script_helper(_: &Context, h: &Helper, _: &Handlebars, rc: &mut RenderContext) -> Result<(), RenderError> {
    let js_links = try!(h.param(0)
            .and_then(|v| v.value().as_array())
            .ok_or(RenderError::new("|> script_helper - param 1 with array type is required")));
    let mut js = "".to_owned();
    for link in js_links.iter() {
        let link = try!(link
                    .as_string()
                    .ok_or(RenderError::new("|> script_helper - array param with string type is required")));
            js = format!("{}\t<script type=\"text/javascript\" charset=\"utf-8\" src=\"{}\"></script>\n", js, link);
    }
    try!(rc.writer.write(js.into_bytes().as_ref()));
    Ok(())
}

fn active_page(_: &Context, h: &Helper, _: &Handlebars, rc: &mut RenderContext) -> Result<(), RenderError> {
    let exact_page = try!(h.param(0)
            .and_then(|v| v.value().as_string())
            .ok_or(RenderError::new("|> active_page - param 1 with string type is required")));
    let active_page = try!(h.param(1)
            .and_then(|v| v.value().as_string())
            .ok_or(RenderError::new("|> active_page - param 2 with string type is required")));
    let mut active = "".to_owned();
    if exact_page == active_page {
        active = "active".to_owned();
    }
    try!(rc.writer.write(active.into_bytes().as_ref()));
    Ok(())
}
