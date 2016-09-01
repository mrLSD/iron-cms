extern crate iron;
extern crate iron_cms;

use iron::{Iron, Chain};

fn main() {
    let routes = iron_cms::routes();
    let mut chain = Chain::new(routes);
    chain.link_after(iron_cms::template());
    chain.link_after(iron_cms::Error404);
    Iron::new(chain).http("localhost:3000").unwrap();
}
