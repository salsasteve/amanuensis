use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::messages)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Message {
    pub id: i32,
    pub name: String,
    pub username: String,
    pub body: String,
    pub exercise_type: String,
    pub photo: Vec<u8>,
}
