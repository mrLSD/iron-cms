extern crate iron;
extern crate router;
extern crate handlebars_iron as hbs;
extern crate handlebars;
extern crate rustc_serialize;
extern crate staticfile;
extern crate mount;
extern crate time;
#[macro_use]
extern crate maplit;

pub mod middleware;

mod admin;
mod frontend;

//pub use middleware::error404;
use router::Router;

use staticfile::Static;
#[cfg(feature = "cache")]
use staticfile::Cache;
use mount::Mount;
use std::path::Path;
#[cfg(feature = "cache")]
use time::Duration;

/// Routes aggregator.
/// It accamilate all posible routes
pub fn routes() -> Mount {
    // Init router
    let mut routes = Router::new();

    // Add routes
    frontend::add_routes(&mut routes);
    admin::add_routes(&mut routes);

    // Add static router
    let mut mount = Mount::new();
//    mount
//        .mount("/", routes)
//        .mount("/assets/", Static::new(Path::new("static"))
//            .cache(Duration::days(30)));
    mount
        .mount("/", routes)
        .mount("/assets/", Static::new(Path::new("static")));
    mount
}
