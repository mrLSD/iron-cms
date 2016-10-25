use iron::prelude::*;
//use std::collections::BTreeMap;
use rustc_serialize::json::{ToJson};
use super::{Render, BaseDataMap, RenderResult};

fn default_param() -> BaseDataMap {
    btreemap! {
        "module".to_string() => "main".to_json(),
    }
}

pub fn main_handler(_: &mut Request) -> RenderResult {
    Render::new("admin/main/index", default_param())
}
