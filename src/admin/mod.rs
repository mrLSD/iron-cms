extern crate time;

use router::Router;
use iron::prelude::*;
use iron::status;
use hbs::{Template};
use rustc_serialize::json::{ToJson};
use std::collections::BTreeMap;

pub fn add_routes(routes: &mut Router) -> &mut Router {
    routes.get("/admin/login.html", admin_handler, "admin_login");
    routes
}

fn admin_handler(_: &mut Request) -> IronResult<Response> {
    let mut resp = Response::new();

    let time = time::now();
    let t = format!("Current date: {}", time.strftime("%Y-%m-%d %T").unwrap());

    let mut data = BTreeMap::new();
    data.insert("name".to_string(), "mrLSD".to_json());
    data.insert("date".to_string(), t.to_json());

    resp.set_mut(Template::new("admin/hello", data)).set_mut(status::Ok);
    Ok(resp)
}