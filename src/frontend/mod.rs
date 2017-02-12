use router::Router;
use iron::prelude::*;
use iron::status;
use hbs::{Template};

pub fn add_routes(routes: &mut Router) -> &mut Router {
    routes.get("/", index_handler, "index_handler");
    routes
}

fn index_handler(_: &mut Request) -> IronResult<Response> {
    let mut resp = Response::new();
    resp.set_mut(Template::new("frontend/home/index", ())).set_mut(status::Ok);
    Ok(resp)
}