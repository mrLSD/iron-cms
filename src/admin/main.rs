use iron::prelude::*;
use iron::status;
use hbs::{Template};
use std::collections::BTreeMap;
use rustc_serialize::json::{Json, ToJson};

fn default_param() -> BTreeMap<String, Json> {
    btreemap! {
        "module".to_string() => "main".to_json(),
    }
}

pub fn main_handler(_: &mut Request) -> IronResult<Response> {
    let data = default_param();
    let mut resp = Response::new();
    resp.set_mut(Template::new("admin/main/index", data)).set_mut(status::Ok);
    Ok(resp)
}
