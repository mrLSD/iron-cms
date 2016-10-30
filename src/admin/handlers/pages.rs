use iron::prelude::*;
use super::*;

fn default_param() -> BaseDataMap {
    btreemap! {
        "module".to_string() => "pages".to_json(),
    }
}

pub fn get_main(_: &mut Request) -> RenderResult {
    Render::new("admin/pages/index", default_param())
}

pub fn get_create(_: &mut Request) -> RenderResult {
    Render::new("admin/pages/create", default_param())
}

pub fn post_create(req: &mut Request) -> RenderResult {
    use params::{Params};
    let _val = itry!(req.get_ref::<Params>());
//
//    let _vld2 = btreemap! {
//        "requiered".to_string() => true.to_json(),
//        "field".to_string() => "name".to_json(),
//    };
//    let json_obj: Json = Json::Object(_vld2);
//    let json_str: String = json_obj.to_string();
//    let _decoded: Validator<String> = json::decode(&json_str).unwrap();
//    if let Err(err) = _decoded.validate(val.find(&["name"])) {
//        println!("res: {:?}", err.errors);
//    }
//    if let Err(err) = _decoded.validate(val.find(&["age"])) {
//        println!("res: {:?}", err.errors);
//    }
    Render::new("admin/pages/create", default_param())
}