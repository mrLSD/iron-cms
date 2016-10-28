use iron::prelude::*;
use rustc_serialize::json::{ToJson};
use super::{Render, BaseDataMap, RenderResult};
use rustc_serialize::json::{self, Json};

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

    #[derive(RustcDecodable, Debug)]
    struct Validator<T> {
        requiered: Option<bool>,
        empty: Option<bool>,
        min: Option<u32>,
        max: Option<u32>,
        dafault: Option<T>,
        field: String,
    }

    impl<T> Validator<T> {
        fn validate(&self, val: Option<&Value>) -> Vec<String> {
            let mut err = vec!();
            if self.requiered.is_some() {
                if val.is_none() {
                    let msg = format!("Field requiered: {}", self.field);
                    err.push(String::from(msg));
                }
            }
            err
        }
    };
    let _vld2 = btreemap! {
        //"requiered".to_string() => true.to_json(),
        "field".to_string() => "name".to_json(),
    };
    let json_obj: Json = Json::Object(_vld2);
    let json_str: String = json_obj.to_string();
    let _decoded: Validator<String> = json::decode(&json_str).unwrap();
    let _s = _decoded.validate(val.find(&["name"]));
    println!("res: {:?}", _s);

    Render::new("admin/pages/create", default_param())
}
