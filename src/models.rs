use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::messages)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Message {
    pub id: i32,
    pub name: Option<String>,
    pub username: String,
    pub body: Option<String>,
    pub exercise_type: Option<String>,
    pub photo: Option<Vec<u8>>,
}
