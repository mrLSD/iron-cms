#[macro_use] extern crate nickel;

use nickel::{Nickel, Mountable, StaticFilesHandler};

/// Build all routers rule
pub fn routers(server: &mut Nickel<()>) {
    server.mount("/", StaticFilesHandler::new("assets/"));
}
