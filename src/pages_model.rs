use schema::pages;

#[derive(Queryable)]
pub struct Page {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}

#[derive(Insertable)]
#[table_name="pages"]
pub struct NewPage {
    pub title: String,
    pub body: String,
    pub published: bool,
}
