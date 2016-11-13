use params::{Map};
use super::*;

use diesel::prelude::*;
use diesel;

#[derive(RustcDecodable, Debug)]
pub struct Pages {
    pub title: String,
    pub body: String,
    pub published: bool,
}

pub struct PagesTable {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}

table! {
    tbl_pages {
        id -> Integer,
        title -> Varchar,
        body -> Text,
        published -> Bool,
    }
}

Insertable! {
    (tbl_pages)
    struct Pages {
        title: String,
        body: String,
        published: bool,
    }
}

Queryable! {
    struct PagesTable {
        id: i32,
        title: String,
        body: String,
        published: bool,
    }
}

pub fn validate(values: &Map) -> ValidateResults {
    ValidateResults(vec!(
        Validator::<String>::new(btreemap! {
            "requiered".to_string() => true.to_json(),
            "vtype".to_string() => "string".to_json(),
        }).validate("title".to_string(), values.find(&["pages", "title"])),

        Validator::<String>::new(btreemap! {
            "requiered".to_string() => true.to_json(),
            "default".to_string() => "".to_json(),
            "vtype".to_string() => "string".to_json(),
        }).validate("body1".to_string(), values.find(&["Contents", "section"])),

        Validator::<bool>::new(btreemap! {
            "default".to_string() => false.to_json(),
            "vtype".to_string() => "bool".to_json(),
        }).validate("published".to_string(), values.find(&["published"])),
    ))
}

pub fn init(values: BaseDataMap) -> Pages {
    values.decode()
}

// Create new Page
pub fn create(conn: &ConnectionPool, values: BaseDataMap) -> InsertResult {
    let new_page: Pages = self::init(values);
    diesel::insert(&new_page).into(tbl_pages::table).execute(&**conn)
}

// List pages
pub fn list(conn: &ConnectionPool) -> Vec<PagesTable> {
    tbl_pages::table
        .limit(5)
        .load::<PagesTable>(&**conn)
        .expect("Error loading page")
}
