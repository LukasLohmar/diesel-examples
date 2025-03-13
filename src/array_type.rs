use std::any::Any;

use diesel::{deserialize::{self, FromSql, FromSqlRow}, expression::AsExpression, serialize::{self, Output, ToSql}, sql_types::{HasSqlType, SqlType}, sqlite::{Sqlite, SqliteValue}};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Copy, Clone, Debug, Serialize, Deserialize, SqlType)]
#[diesel(sqlite_type(name = "Binary"))]
pub struct ArrayType<ST: 'static>(ST);

#[derive(Debug, Clone, PartialEq, FromSqlRow, AsExpression, Serialize, Deserialize)]
#[diesel(sql_type = ArrayType<T>)]
pub struct Array<T>(pub Vec<T>);

impl<T, ST> FromSql<ArrayType<ST>, Sqlite> for Array<T>
where
    T: 'static + FromSql<ST, Sqlite> + Any + Clone,
{
    fn from_sql(bytes: SqliteValue<'_, '_, '_>) -> deserialize::Result<Self> {
        let value = <serde_json::Value as FromSql<diesel::sql_types::Jsonb, Sqlite>>::from_sql(bytes);
        let mut result: Vec<T> = vec![];

        match value {
            Ok(value) => {
                if value.is_null() {
                    return Ok(Self(result));
                }

                if !value.is_array() {
                    return Ok(Self(result));
                }

                match value.as_array() {
                    Some(value) => {
                        for value in value {
                            if let Some(value) = value.as_bool() {
                                let value = (&value as &dyn Any).downcast_ref::<T>();

                                match value {
                                    Some(value) => result.push(value.to_owned()),
                                    None => return Ok(Self(result)),
                                }
                            } else if let Some(value) = value.as_i64() {
                                let value = value as i32;
                                let value = (&value as &dyn Any).downcast_ref::<T>();

                                match value {
                                    Some(value) => result.push(value.to_owned()),
                                    None => return Ok(Self(result)),
                                }
                            } else if let Some(value) = value.as_f64() {
                                let value = value as f32;
                                let value = (&value as &dyn Any).downcast_ref::<T>();

                                match value {
                                    Some(value) => result.push(value.to_owned()),
                                    None => return Ok(Self(result)),
                                }
                            } else if let Some(value) = value.as_str() {
                                let value = value.to_owned();
                                let value = (&value as &dyn Any).downcast_ref::<T>();

                                match value {
                                    Some(value) => result.push(value.to_owned()),
                                    None => return Ok(Self(result)),
                                }
                            }
                        }
                    },
                    None => return Ok(Self(result)),
                }

                Ok(Self(result))
            },
            Err(_) => Ok(Self(result)),
        }
    }
}

impl<T, ST> ToSql<ArrayType<ST>, Sqlite> for Array<T>
where
    T: Clone,
    T: ToSql<diesel::sql_types::Jsonb, Sqlite>,
    T: ToSql<ST, Sqlite> + Serialize
{
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Sqlite>) -> serialize::Result {
        let value = json!(self.0).to_string();

        out.set_value(value);

        Ok(serialize::IsNull::No)
        // <serde_json::Value as ToSql::<diesel::sql_types::Jsonb, Sqlite>>::to_sql(json!(self.0), out);
    }
}
