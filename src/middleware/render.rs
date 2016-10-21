#![allow(dead_code)]
use hbs::{HandlebarsEngine, DirectorySource};
use handlebars::{Handlebars, RenderError, RenderContext, Helper, Context};
use time;

/// Init Template renderer and add Tempaltes path.
/// It invoke to after middleware
pub fn template_render(paths: Vec<&str>) -> HandlebarsEngine {
    let mut hregistry = Handlebars::new();

    // Add helpers
    hregistry.register_helper("link", Box::new(link_helper));
    hregistry.register_helper("script", Box::new(script_helper));

    let mut template = HandlebarsEngine::from(hregistry);

    // add a directory source, all files with .html suffix
    // will be loaded as template
    for path in paths.iter() {
        template.add(Box::new(DirectorySource::new(path, ".html")));
    }

    // load templates from all registered sources
    if let Err(r) = template.reload() {
        panic!("{:?}", r);
    }
    template
}

/// Css link Helper
/// usege: `{{#link ...}}`
fn link_helper(_: &Context, h: &Helper, _: &Handlebars, rc: &mut RenderContext) -> Result<(), RenderError> {
    let mut css = "".to_owned();
    if h.params().len() == 1 {
        let value = h.param(0).unwrap().value();
        if value.is_array() {
            let css_links = value.as_array().unwrap();
            for link in css_links.iter() {
                if link.is_string() {
                    css = format!("{}\t<link rel=\"stylesheet\" type=\"text/css\" href=\"{}\">\n", css, link.as_string().unwrap());
                } else {
                    helper_log(&h, rc, "wrong array value type - string expected")
                }
            }
        } else {
            helper_log(&h, rc, "wrong type - array expected")
        }
    } else {
        helper_log(&h, rc, "wrong params count")
    }
    try!(rc.writer.write(css.into_bytes().as_ref()));
    Ok(())
}

/// Js link Helper
/// usege: `{{#script ...}}`
fn script_helper(_: &Context, h: &Helper, _: &Handlebars, rc: &mut RenderContext) -> Result<(), RenderError> {
    let mut js = "".to_owned();
    if h.params().len() == 1 {
        let value = h.param(0).unwrap().value();
        if value.is_array() {
            let js_links = value.as_array().unwrap();
            for link in js_links.iter() {
                if link.is_string() {
                    js = format!("{}\t<script type=\"text/javascript\" charset=\"utf-8\" src=\"{}\"></script>\n", js, link.as_string().unwrap());
                } else {
                    helper_log(&h, rc, "wrong array value type - string expected")
                }
            }
        } else {
            helper_log(&h, rc, "wrong type - array expected")
        }
    } else {
        helper_log(&h, rc, "wrong params count")
    }
    try!(rc.writer.write(js.into_bytes().as_ref()));
    Ok(())
}

fn helper_log(h: &Helper, rc: &mut RenderContext, message: &str) {
    println!("{} {:?} |> Helper[{}]: {}",
                time::now().strftime("%Y-%m-%d %T").unwrap(),
                rc.current_template,
                h.name(),
                message);
}
