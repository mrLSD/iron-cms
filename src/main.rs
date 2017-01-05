<<<<<<< HEAD
#[macro_use] extern crate nickel;
extern crate iron_cms;

use nickel::{Nickel, Options};

fn main() {
    let mut server = Nickel::new();
    server.options = Options::default()
                     .output_on_listen(false)
                     .thread_count(Some(100));

    // Middlewars
    iron_cms::routers(&mut server);

    server.listen("127.0.0.1:3000").unwrap();
=======
extern crate iron;
extern crate iron_cms;

use iron::{Iron, Chain};

fn main() {
    // Add routers
    let mut chain = Chain::new(iron_cms::routes());
    // Add db middleware
    chain.link_before(iron_cms::middleware::db("postgres://postgres:qwe123qwe123@172.18.0.2:5432/test_db"));
    // Add Template renderer and views path
    chain.link_after(iron_cms::middleware::template_render(vec!["./views/"]));
    // Add error-404 handler
    chain.link_after(iron_cms::middleware::Error404);
    // Start application
    Iron::new(chain).http("localhost:3000").unwrap();
>>>>>>> 72d372edd2b2daf38944fa58126bdb5ce740652f
}
