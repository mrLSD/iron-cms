use schema::pages;

#[derive(Queryable)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub published: bool,
}

#[derive(Insertable)]
#[table_name="pages"]
pub struct NewPost<'a> {
    pub title: &'a str,
    pub published: bool,
}
