use diesel::prelude::*;
use diesel::sqlite::Sqlite;
use diesel::{Queryable, Selectable};

use crate::array_type::Array;

#[derive(Debug, Selectable, Queryable)]
#[diesel(table_name = crate::schema::test)]
#[diesel(check_for_backend(Sqlite))]
pub struct Test {
    pub id: i32,
    pub bool_value: Option<Array<bool>>,
    pub int_value: Option<Array<i32>>,
    pub float_value: Option<Array<f32>>,
    pub string_value: Option<Array<String>>
}
