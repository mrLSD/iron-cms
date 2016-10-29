use iron::prelude::*;
use super::*;

fn default_param() -> BaseDataMap {
    btreemap! {
        "module".to_string() => "main".to_json(),
    }
}

pub fn get_main(_: &mut Request) -> RenderResult {
    Render::new("admin/main/index", default_param())
}
