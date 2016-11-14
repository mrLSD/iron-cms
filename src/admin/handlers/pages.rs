use iron::prelude::*;
use super::*;
use iron::modifiers::*;
use iron::status;

fn default_param(data: &BaseDataMap) -> BaseDataMap {
    let mut bm = btreemap! {
        "module".to_string() => "pages".to_json(),
    };
    bm.append(&mut data.to_owned());
    bm
}

pub fn get_main(_: &mut Request) -> RenderResult {
    Render::new("admin/pages/index", default_param(&btreemap! {}))
}

pub fn get_create(_: &mut Request) -> RenderResult {
    Render::new("admin/pages/create", default_param(&btreemap! {}))
}

pub fn get_show(req: &mut Request) -> RenderResult {
    let conn = req.db_conn();
    models::pages::list(&conn);
    Render::new("admin/pages/create", default_param(&btreemap! {}))
}

pub fn post_create(req: &mut Request) -> RenderResult {
    use params::{Params};

    let conn = req.db_conn();
    let values = itry!(req.get::<Params>());
    let validate = models::pages::validate(&values);
    if let Some(err) = validate.get_errors() {
        let b = btreemap! {"result".to_string() => validate.to_json()};
        println!("Validation Errors: {:?}\n\n {:?}", err, validate);
        return Render::new("admin/pages/create", default_param(&b));
    } else {
        itry!(models::pages::create(&conn, validate.get_values()));
        return Ok(Response::with((status::Found, Redirect(url_for!(req, "admin_pages_main")))))
    }

    Render::new("admin/pages/create", default_param(&btreemap! {}))
}