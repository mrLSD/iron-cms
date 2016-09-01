#![allow(dead_code)]
use hbs::{HandlebarsEngine, DirectorySource};
use handlebars::{Handlebars, RenderError, RenderContext, Helper, Context};

/// Init Template renderer and add Tempaltes path.
/// It invoke to after middleware
pub fn template_render(paths: Vec<&str>) -> HandlebarsEngine {
    let mut template = HandlebarsEngine::new();

    // Add helpers
    template.register_helper("link", Box::new(link_helper));
    template.register_helper("script", Box::new(script_helper));

    // add a directory source, all files with .html suffix
    // will be loaded as template
    for path in paths.iter() {
        template.add(Box::new(DirectorySource::new(path, ".html")));
    }

    // load templates from all registered sources
    if let Err(r) = template.reload() {
        panic!("{}", r);
    }
    template
}

/// Css link Helper
/// usege: `{{#link ...}}`
fn link_helper(_: &Context, h: &Helper, _: &Handlebars, rc: &mut RenderContext) -> Result<(), RenderError> {
    let css_links = h.param(0).unwrap().value().as_array().unwrap();
    let mut css = "".to_owned();
    for link in css_links.iter() {
        css = format!("{}\t<link rel=\"stylesheet\" type=\"text/css\" href=\"{}\">\n", css, link);
    }
    try!(rc.writer.write(css.into_bytes().as_ref()));
    Ok(())
}

/// Js link Helper
/// usege: `{{#script ...}}`
fn script_helper(_: &Context, h: &Helper, _: &Handlebars, rc: &mut RenderContext) -> Result<(), RenderError> {
    let js_link = h.param(0).unwrap().value().as_string().unwrap();
    let js = format!("\t<script type=\"text/javascript\" charset=\"utf-8\" src=\"{}\"></script>\n", js_link);
    try!(rc.writer.write(js.into_bytes().as_ref()));
    Ok(())
}