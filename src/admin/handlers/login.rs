use iron::prelude::*;
use super::*;
use time;

pub fn get_main(_: &mut Request) -> RenderResult {
    let time = time::now();
    let t = format!("Current date: {}", time.strftime("%Y-%m-%d %T").unwrap());

    let data = btreemap! {
        "name".to_string() => "Iron CMS".to_string(),
        "date".to_string() => t.to_string()
    };

    Render::new("admin/main/index", data)
}
