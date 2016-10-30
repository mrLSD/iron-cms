use rustc_serialize::json::{ToJson};
use params::{Map};
use super::*;

struct _Pages {
    pub title: String,
    pub published: bool,
}

pub fn validate(values: &Map) -> VRes {
    vec!(
        Validator::<String>::new(btreemap! {
            "requiered".to_string() => true.to_json(),
        }).validate("title".to_string(), values.find(&["title"])),

        Validator::<bool>::new(btreemap! {
            "default".to_string() => false.to_json(),
        }).validate("published".to_string(), values.find(&["published"])),
    )
}
