use params::{Map};
use super::*;

use diesel::prelude::*;
use diesel;
use diesel::result::Error;
use r2d2;
use r2d2_diesel;
use diesel::pg::PgConnection;
type ConnectionPool = r2d2::PooledConnection<r2d2_diesel::ConnectionManager<PgConnection>>;

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
        title ->  Varchar,
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
    values.decode()
}

// Create new Page
pub fn create(conn: &ConnectionPool, values: BaseDataMap) -> Result<usize, Error> {
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
