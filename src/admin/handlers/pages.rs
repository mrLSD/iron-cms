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

pub fn get_show(_: &mut Request) -> RenderResult {
    models::pages::show();
    Render::new("admin/pages/create", default_param())
}

pub fn post_create(req: &mut Request) -> RenderResult {
    use params::{Params};
    let values = itry!(req.get_ref::<Params>());
    let validate = models::pages::validate(values);
    if let Some(err) = validate.get_errors() {
        println!("Validation Errors: {:?}", err);
    }
    let model = models::pages::init(validate.get_values());
    println!("{:?}", model);

    Render::new("admin/pages/create", default_param())
}