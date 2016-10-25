use router::Router;
use iron::prelude::*;
use time;
pub use super::middleware::*;

mod main;
mod pages;

pub fn add_routes(routes: &mut Router) -> &mut Router {
    routes.get("/admin.html", main::main_handler, "admin_main");
    routes.get("/admin/pages/index.html", pages::main_handler, "admin_pages_main");
    routes.get("/admin/login.html", admin_login_handler, "admin_login");
    routes
}

fn admin_login_handler(_: &mut Request) -> RenderResult {
    let time = time::now();
    let t = format!("Current date: {}", time.strftime("%Y-%m-%d %T").unwrap());

    let data = btreemap! {
        "name".to_string() => "mrLSD".to_string(),
        "date".to_string() => t.to_string()
    };

    Render::new("admin/main/index", data)
}
