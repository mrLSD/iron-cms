extern crate iron;
extern crate iron_cms;

use iron::{Iron, Chain};

fn main() {
    // Add routers
    let mut chain = Chain::new(iron_cms::routes());
    // Add Template renderer and views path
    let paths = vec!["./views/"];
    chain.link_after(iron_cms::middleware::template_render(paths));
    // Add error-404 handler
    chain.link_after(iron_cms::middleware::Error404);
    // Start application
    Iron::new(chain).http("localhost:3000").unwrap();
}
