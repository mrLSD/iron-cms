//! # Iron CMS
//! CMS based on Iron Framework for **Rust**.
#[macro_use] extern crate iron;
#[macro_use] extern crate router;
#[macro_use] extern crate maplit;
#[macro_use] extern crate diesel;
extern crate handlebars_iron as hbs;
extern crate handlebars;
extern crate rustc_serialize;
extern crate staticfile;
extern crate mount;
extern crate time;
extern crate params;

extern crate iron_diesel_middleware;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate regex;

/// Base middleware for CMS
pub mod middleware;

mod admin;
mod frontend;

use router::Router;

use staticfile::Static;
#[cfg(feature = "cache")]
use staticfile::Cache;
use mount::Mount;
use std::path::Path;
#[cfg(feature = "cache")]
use time::Duration;

/// Routes aggregator.
/// It accumulate all posible routes for CMS.
/// ## How to use
/// ```
/// extern crate iron;
/// extern crate iron_cms;
/// use iron::{Iron, Chain};
/// fn main() {
///    // Add routers
///    let mut chain = Chain::new(iron_cms::routes());
///    // Add Template renderer and views path
///    let paths = vec!["./views/"];
///    chain.link_after(iron_cms::middleware::template_render(paths));
///    // Add error-404 handler
///    chain.link_after(iron_cms::middleware::Error404);
///    // Start applocation and other actions
///    // Iron::new(chain).http("localhost:3000").unwrap();
/// }
/// ```
pub fn routes() -> Mount {
    // Init router
    let mut routes = Router::new();

    // Add routes
    frontend::add_routes(&mut routes);
    admin::add_routes(&mut routes);

    // Add static router
    let mut mount = Mount::new();
    mount
        .mount("/", routes)
        .mount("/assets/", Static::new(Path::new("static")));
//      .cache(Duration::days(30)));
    mount
}
