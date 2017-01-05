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
/// it's included paths ./views/**/*
use iron::prelude::*;
use iron::status;
use hbs::{HandlebarsEngine, DirectorySource};
use handlebars::{Handlebars, RenderError, RenderContext, Helper, Context};
use std::error::Error;
use std::collections::BTreeMap;
use rustc_serialize::json::{self, Json, ToJson};
use rustc_serialize::json::DecoderError::*;
use rustc_serialize::Decodable;
use hbs::{Template};
use handlebars::Renderable;

const DEBUG_RENDER: bool = true;

/// Alias for Basic Data struct
pub type BaseDataMap = BTreeMap<String, Json>;
/// Alias for basic Iron Response Result
pub type RenderResult = IronResult<Response>;
/// Templфte Render strшct
pub struct Render {
    pub data : BaseDataMap
}

/// BaseDataMap Json decoder trait
pub trait BaseDataMapDecoder {
    fn decode<J: Decodable>(&self) -> J;
}

/// Omplementation for Jscond decoding
/// from BaseDataMap that implemented Decodable trait
/// to specific generic type.
/// It useful for struct init via BaseDataMap data.
/// For Example - models.
impl BaseDataMapDecoder for BaseDataMap {
    /// Json decoder for BaseDataMap
    fn decode<J: Decodable>(&self) -> J {
        let json_obj: Json = Json::Object(self.to_owned());
        match json::decode(&json_obj.to_string()) {
            Ok(decoded) => decoded,
            Err(err) => {
                let msg = match err {
                    ParseError(_) => "JSON parse error",
                    ExpectedError(_, _) => "Validation field expected (wrong type)",
                    MissingFieldError(_) => "Validation field missing",
                    _ => "Other error",
                };
                panic!("\
              \n\n |> Validator::new error: {:?}\
                \n |> Validation fields: {:?}\
                \n |> Message: {}\
                \n |> At source code: => ", err, json_obj, msg);
            }
        }
    }
}

/// Basic render with StatusOK tempalte name and data
/// basic usage:
/// `Render::new("my/template/path", ())``
impl Render {
    /// Render Template file with status 200
    pub fn new<T: ToJson>(name: &str, data: T) -> RenderResult {
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
    hregistry.register_helper("active", Box::new(active_page_helper));
    hregistry.register_helper("ifeq", Box::new(ifeq_helper));
    hregistry.register_helper("ifgt", Box::new(ifgt_helper));

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

/// Active helper.
/// It checking is value same with exacted value
/// usege: `{{#active "pages" module }}{{/active}}`
/// It should pre-init value at Handler!
fn active_page_helper(_: &Context, h: &Helper, _: &Handlebars, rc: &mut RenderContext) -> Result<(), RenderError> {
    println!("ACTIVE=> {:?} {:?}\n", h.param(0), h.param(1));
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

fn ifeq_helper(ctx: &Context, h: &Helper, hbs: &Handlebars, rc: &mut RenderContext) -> Result<(), RenderError> {
    let value = try!(h.param(0)
            .and_then(|v| Some(v.value()) )
            .ok_or(RenderError::new("|> ifeq_helper - param 1 with is required")));
    let eq_field = try!(h.param(1)
            .and_then(|v| Some(v.value()) )
            .ok_or(RenderError::new("|> ifeq_helper - param 2 with is required")));
    println!("==> {:?}\n\n", h.param(0));

    let is_true = value == eq_field;

    if is_true {
        if let Some(tpl) = h.template() {
            tpl.render(ctx, hbs, rc)?;
        }
    } else {
        if let Some(tpl) = h.inverse() {
            tpl.render(ctx, hbs, rc)?;
        }
    }
    Ok(())
}

fn ifgt_helper(ctx: &Context, h: &Helper, hbs: &Handlebars, rc: &mut RenderContext) -> Result<(), RenderError> {
    let mut active = "".to_owned();;
    let value = try!(h.param(0)
            .and_then(|v| Some(v.value()) )
             .ok_or(RenderError::new("|> ifgt_helper - param 1 withis required")));
    if DEBUG_RENDER {
        let eq_field = try!(h.param(1)
            .and_then(|v| Some(v.value()))
            .ok_or(RenderError::new("|> ifgt_helper - param 2 with is required")));
        println!("IFGT==> {:?}\n\n", value);
        active = "5 23".to_owned();
        println!("IFGT==>>> {:?}\n\n", value);
        let (_, _, _) = (eq_field, hbs, ctx);
    }
    try!(rc.writer.write(active.into_bytes().as_ref()));

    Ok(())
}