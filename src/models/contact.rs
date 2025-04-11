use diesel::prelude::*;
use diesel::sqlite::Sqlite;

#[derive(Debug, Selectable, Queryable)]
#[diesel(table_name = crate::schema::contact)]
#[diesel(check_for_backend(Sqlite))]
pub struct Contact {
    pub id: i32,
    pub name: String,
}
