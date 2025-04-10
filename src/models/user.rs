use anyhow::Error;
use diesel::prelude::*;
use diesel::sqlite::Sqlite;
use diesel::{dsl, Queryable, Selectable};
use diesel::helper_types::{InnerJoinQuerySource};
use diesel::query_source::Alias;
use crate::models::user::filter::Filter;

diesel::alias!(crate::schema::user as self_alias: SelfAlias, crate::schema::user as created_by_alias: CreatedByAlias, crate::schema::contact as contact_alias: ContactAlias);

type UserContactJoinConstraint = dsl::Eq<crate::schema::user::contact_id, crate::schema::contact::id>;

type UserFilterSqlite<'a> = Box<dyn BoxableExpression<InnerJoinQuerySource<crate::schema::user::dsl::user, Alias<ContactAlias>, UserContactJoinConstraint>, Sqlite, SqlType = diesel::sql_types::Bool> + 'a>;

#[derive(Debug, Selectable, Queryable)]
#[diesel(table_name = crate::schema::user)]
#[diesel(check_for_backend(Sqlite))]
pub struct User {
    pub id: i32,
    pub contact_id: i32,
    pub created_by: i32,
}

impl User {
    fn filter<'a>(filter: &'a Filter) -> Result<UserFilterSqlite<'a>, Error> {
        match filter.column {
            filter::Column::Id => {
                let value = filter.value.parse::<i32>()?;

                let expression = Box::new(crate::schema::user::id.eq(value));

                Ok(expression)
            }
            filter::Column::ContactId => {
                let value = filter.value.parse::<i32>()?;

                let expression = Box::new(crate::schema::user::id.eq(value));

                Ok(expression)
            }
            filter::Column::ContactName => {
                let value = filter.value;

                let expression = Box::new(contact_alias.field(crate::schema::contact::name).eq(value));

                Ok(expression)
            }
            filter::Column::CreatedById => {
                todo!("")
            }
            filter::Column::CreatedBy => {
                todo!("")
            }
        }
    }

    pub fn execute_filter<'a>(connection: &mut SqliteConnection, filter: &'a Filter) -> Result<Vec<Self>, Error> {
        let mut query = crate::schema::user::dsl::user
            .inner_join(contact_alias.on(crate::schema::user::contact_id.eq(contact_alias.field(crate::schema::contact::id))))
            .into_boxed();

        let filter = Self::filter(filter)?;

        query = query.filter(filter);

        let results = query.select(Self::as_select()).get_results(connection)?;

        Ok(results)
    }
}

pub mod filter {
    pub enum Column {
        Id,
        ContactId,
        ContactName,
        CreatedById,
        CreatedBy,
    }

    pub struct Filter<'a> {
        pub value: &'a str,
        pub column: Column,
    }
}
