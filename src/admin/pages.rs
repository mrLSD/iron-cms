use iron::prelude::*;
use rustc_serialize::json::{ToJson};
use super::{Render, BaseDataMap, RenderResult};

fn default_param() -> BaseDataMap {
    btreemap! {
        "module".to_string() => "pages".to_json(),
    }
}

pub fn main_handler(_: &mut Request) -> RenderResult {
    Render::new("admin/pages/index", default_param())
}
