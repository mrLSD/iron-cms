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

pub fn create_handler(req: &mut Request) -> RenderResult {
    use params::{Params, Value};
    let val = itry!(req.get_ref::<Params>());

    let _vld2 = btreemap! {
        "requiered".to_string() => true.to_json(),
        "field".to_string() => "name".to_json(),
    };
    let json_obj: Json = Json::Object(_vld2);
    let json_str: String = json_obj.to_string();
    let _decoded: Validator<String> = json::decode(&json_str).unwrap();
    if let Err(err) = _decoded.validate(val.find(&["name"])) {
        println!("res: {:?}", err.errors);
    }
    if let Err(err) = _decoded.validate(val.find(&["age"])) {
        println!("res: {:?}", err.errors);
    }

    Render::new("admin/pages/create", default_param())
}
