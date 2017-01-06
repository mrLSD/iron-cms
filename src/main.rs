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
}
