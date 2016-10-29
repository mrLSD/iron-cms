use router::Router;
use iron::prelude::*;
pub use super::middleware::*;

pub fn add_routes(routes: &mut Router) -> &mut Router {
    routes.get("/admin.html", main::main_handler, "admin_main");
    routes.get("/admin/pages.html", pages::main_handler, "admin_pages_main");
    routes.get("/admin/pages/create.html", pages::create_handler, "admin_pages_create");
    routes.get("/admin/login.html", admin_login_handler, "admin_login");
    routes.post("/admin/pages/create.html", pages::create_handler, "admin_pages_create");
    routes
}

use time;

fn admin_login_handler(_: &mut Request) -> RenderResult {
    let time = time::now();
    let t = format!("Current date: {}", time.strftime("%Y-%m-%d %T").unwrap());

    let data = btreemap! {
        "name".to_string() => "Iron CMS".to_string(),
        "date".to_string() => t.to_string()
    };

    Render::new("admin/main/index", data)
}
