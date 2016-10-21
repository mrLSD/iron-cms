use router::Router;
use iron::prelude::*;
use iron::status;
use hbs::{Template};
use time;

pub fn add_routes(routes: &mut Router) -> &mut Router {
    routes.get("/admin.html", admin_main_handler, "admin_main");
    routes.get("/admin/login.html", admin_login_handler, "admin_login");
    routes
}

fn admin_main_handler(_: &mut Request) -> IronResult<Response> {
    let mut resp = Response::new();
    resp.set_mut(Template::new("admin/layout/main", ())).set_mut(status::Ok);
    Ok(resp)
}

fn admin_login_handler(_: &mut Request) -> IronResult<Response> {
    let mut resp = Response::new();

    let time = time::now();
    let t = format!("Current date: {}", time.strftime("%Y-%m-%d %T").unwrap());

    let data = btreemap! {
        "name".to_string() => "mrLSD".to_string(),
        "date".to_string() => t.to_string()
    };

    resp.set_mut(Template::new("admin/hello", data)).set_mut(status::Ok);
    Ok(resp)
}