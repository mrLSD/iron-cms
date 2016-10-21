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
#![allow(dead_code)]
use hbs::{HandlebarsEngine, DirectorySource};
use handlebars::{Handlebars, RenderError, RenderContext, Helper, Context};
use time;

/// Init Template renderer and add Tempaltes path.
/// It invoke to after middleware
pub fn template_render(paths: Vec<&str>) -> HandlebarsEngine {
    // First init Handlebars
    let mut hregistry = Handlebars::new();

    // Add helpers to Handlebars
    hregistry.register_helper("link", Box::new(link_helper));
    hregistry.register_helper("script", Box::new(script_helper));

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
        panic!("{:?}", r);
    }
    template
}

/// Css link Helper
/// usege: `{{#link ["some/url1", "some/url2"]}}{{/link}}`
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
/// usege: `{{#script ["some/url1", "some/url2"]}}{{/script}}`
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

/// Helper logger (currently simple stdout)
/// Pretty log with Time, Template name, Helper name, Message
fn helper_log(h: &Helper, rc: &mut RenderContext, message: &str) {
    println!("{} \n\t |> Template:{:?} \n\t |> Helper[{}] \n\t |> Message: {}",
                time::now().strftime("%Y-%m-%d %T").unwrap(),
                rc.current_template,
                h.name(),
                message);
}
