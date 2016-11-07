//use rustc_serialize::json::{ToJson};
use params::{Map};
use super::*;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

#[derive(RustcDecodable, Debug)]
pub struct Pages {
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

pub fn init(values: BaseDataMap) -> Pages {
    values.decode()
}

fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
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