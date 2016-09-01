extern crate iron;
extern crate router;
extern crate handlebars_iron as hbs;
extern crate rustc_serialize;
extern crate staticfile;
extern crate mount;
extern crate time;

mod admin;
mod frontend;
mod middleware;

pub use middleware::Error404;
use router::Router;
use hbs::{HandlebarsEngine};

use staticfile::Static;
use mount::Mount;
use std::path::Path;

pub fn routes() -> Mount {
    let mut routes = Router::new();
    frontend::add_routes(&mut routes);
    admin::add_routes(&mut routes);

    let mut mount = Mount::new();
    mount
        .mount("/", routes)
        .mount("/assets/", Static::new(Path::new("static")));
    mount
}

pub fn template() -> HandlebarsEngine {
    let mut tmpl = HandlebarsEngine::new();
    middleware::template(&mut tmpl, "./views/");
    tmpl
}
