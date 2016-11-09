//use rustc_serialize::json::{ToJson};
use params::{Map};
use super::*;
use pages_model::*;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use diesel;
use std::env;

#[derive(RustcDecodable, Debug)]
pub struct Pages {
    pub title: String,
    pub body: String,
    pub published: bool,
}

pub fn validate(values: &Map) -> ValidateResults {
    ValidateResults(vec!(
        Validator::<String>::new(btreemap! {
            "requiered".to_string() => true.to_json(),
            "vtype".to_string() => "string".to_json(),
        }).validate("title".to_string(), values.find(&["title"])),
        Validator::<String>::new(btreemap! {
            "default".to_string() => "".to_json(),
            "vtype".to_string() => "string".to_json(),
        }).validate("body".to_string(), values.find(&["body"])),
        Validator::<bool>::new(btreemap! {
            "default".to_string() => false.to_json(),
            "vtype".to_string() => "bool".to_json(),
        }).validate("published".to_string(), values.find(&["published"])),
    ))
}

pub fn init(values: BaseDataMap) -> Pages {
    println!("INIT");
    values.decode()
}

fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn create(values: BaseDataMap) -> Page {
    use schema::pages;

    let page = self::init(values);

    let new_page = NewPage {
        title: page.title,
        body: page.body,
        published: page.published,
    };

    let connection = establish_connection();
    diesel::insert(&new_page).into(pages::table)
        .get_result(&connection)
        .expect("Error saving new post")
}

pub fn show() {
    use schema::pages::dsl::*;
    use pages_model::*;

    let connection = establish_connection();
    let results = pages
        .limit(5)
        .load::<Page>(&connection)
        .expect("Error loading page");

    println!("Displaying {} pages", results.len());
    for post in results {
        println!("{}", post.title);
        println!("-----------\n");
        println!("{}", post.body);
    }

}