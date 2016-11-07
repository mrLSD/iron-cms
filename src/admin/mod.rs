use router::Router;
pub use super::middleware::*;

mod handlers;
pub mod models;

pub fn add_routes(routes: &mut Router) -> &mut Router {
    routes.get("/admin.html", handlers::main::get_main, "admin_main");
    routes.get("/admin/pages.html", handlers::pages::get_main, "admin_pages_main");
    routes.get("/admin/pages/create.html", handlers::pages::get_create, "admin_pages_create");
    routes.get("/admin/login.html", handlers::login::get_main, "admin_login");
    routes.get("/admin/pages/show", handlers::pages::get_show, "admin_pages_show");

    routes.post("/admin/pages/create.html", handlers::pages::post_create, "admin_pages_create");
    routes
}
