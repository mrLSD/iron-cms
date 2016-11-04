//use rustc_serialize::json::{ToJson};
use params::{Map};
use super::*;

#[derive(RustcDecodable, Debug)]
struct Pages {
    pub title: String,
    pub published: bool,
}

pub fn validate(values: &Map) -> ValidateResults {
    ValidateResults(vec!(
        Validator::<String>::new(btreemap! {
            "requiered".to_string() => true.to_json(),
            "vtype".to_string() => "string".to_json(),
        }).validate("title".to_string(), values.find(&["title"])),
        Validator::<bool>::new(btreemap! {
            "default".to_string() => false.to_json(),
            "vtype".to_string() => "bool".to_json(),
        }).validate("published".to_string(), values.find(&["published"])),
    ))
}

pub fn init(values: BaseDataMap) {
    println!("{:?}", values);
/*
    let _r = btreemap! {
        "title".to_string() => "pages".to_json(),
        "published".to_string() => "true".to_json(),
    };
    let json_obj: Json = Json::Object(_r);
    let json_str: String = json_obj.to_string();
    if let Some(res) = json_obj.find("published") {
        println!("{:?}", res.as_boolean());
        if let Some(r) = res.as_string() {
            println!("{:?}", r.parse::<bool>());
        }
    }
    let _model: Pages = decode(&json_str).unwrap();

    println!("json: {:?}", _model);

    let json_obj: Json = Json::Object(values);
    let json_str: String = json_obj.to_string();
    println!("json: {}", json_str);
    if let Err(err) = decode::<Pages>(&json_str) {
        println!("Err: {:?}", err);
    }
*/
    //let _model: Pages = decode(&json_str).unwrap();
    //println!("model: {:?}", _model);
}
