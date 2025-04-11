use crate::models::contact::Contact;
use crate::models::user::filter::Filter;
use anyhow::Error;
use diesel::helper_types::InnerJoinQuerySource;
use diesel::prelude::*;
use diesel::query_source::{Alias, AliasedField};
use diesel::sqlite::Sqlite;
use diesel::{Queryable, Selectable, dsl};

diesel::alias!(crate::schema::user as self_alias: SelfAlias, crate::schema::contact as contact_alias: ContactAlias, crate::schema::user as created_by_alias: CreatedByAlias);

type UserContactJoinConstraint = dsl::Eq<
    AliasedField<SelfAlias, crate::schema::user::contact_id>,
    AliasedField<ContactAlias, crate::schema::contact::id>,
>;
type UserCreatedByJoinConstraint = dsl::Eq<
    AliasedField<SelfAlias, crate::schema::user::created_by_id>,
    AliasedField<CreatedByAlias, crate::schema::user::id>,
>;

type UserFilterSqlite<'a> = Box<
    dyn BoxableExpression<
            InnerJoinQuerySource<
                InnerJoinQuerySource<
                    Alias<SelfAlias>,
                    Alias<ContactAlias>,
                    UserContactJoinConstraint,
                >,
                Alias<CreatedByAlias>,
                UserCreatedByJoinConstraint,
            >,
            Sqlite,
            SqlType = diesel::sql_types::Bool,
        > + 'a,
>;

#[derive(Debug, Selectable, Queryable)]
#[diesel(table_name = crate::schema::user)]
#[diesel(check_for_backend(Sqlite))]
pub struct User {
    pub id: i32,
    pub contact_id: i32,
    pub name: String,
    pub created_by_id: i32,
}

impl User {
    fn filter<'a>(filter: &'a Filter) -> Result<UserFilterSqlite<'a>, Error> {
        match filter.column {
            filter::Column::Id => {
                // filter results by id on user table
                let value = filter.value.parse::<i32>()?;

                let expression = Box::new(self_alias.field(crate::schema::user::id).eq(value));

                Ok(expression)
            }
            filter::Column::ContactId => {
                // filter results by contact_id on user table
                let value = filter.value.parse::<i32>()?;

                let expression =
                    Box::new(self_alias.field(crate::schema::user::contact_id).eq(value));

                Ok(expression)
            }
            filter::Column::ContactName => {
                // filter results by name on contact table
                let value = filter.value;

                let expression =
                    Box::new(contact_alias.field(crate::schema::contact::name).eq(value));

                Ok(expression)
            }
            filter::Column::CreatedById => {
                // filter results by contact name
                let value = filter.value.parse::<i32>()?;

                let expression = Box::new(
                    self_alias
                        .field(crate::schema::user::created_by_id)
                        .eq(value),
                );

                Ok(expression)
            }
            filter::Column::CreatedByName => {
                // filter results by name on self-joined user table
                let value = filter.value;

                let expression =
                    Box::new(created_by_alias.field(crate::schema::user::name).eq(value));

                Ok(expression)
            }
        }
    }

    pub fn execute_filter<'a>(
        connection: &mut SqliteConnection,
        filter: &'a Filter,
    ) -> Result<Vec<Self>, Error> {
        let contact_join_constraint = self_alias
            .field(crate::schema::user::contact_id)
            .eq(contact_alias.field(crate::schema::contact::id));
        let created_by_join_constraint = self_alias
            .field(crate::schema::user::created_by_id)
            .eq(created_by_alias.field(crate::schema::user::id));

        let mut query = self_alias
            .inner_join(contact_alias.on(contact_join_constraint))
            .inner_join(created_by_alias.on(created_by_join_constraint))
            .into_boxed();

        let filter = Self::filter(filter)?;

        query = query.filter(filter);

        // query.select(Self::as_select()) wont work here
        let results = query
            .select(self_alias.fields((
                crate::schema::user::id,
                crate::schema::user::contact_id,
                crate::schema::user::name,
                crate::schema::user::created_by_id,
            )))
            .load(connection)?;

        Ok(results)
    }
}

pub mod filter {
    pub enum Column {
        Id,
        ContactId,
        ContactName,
        CreatedById,
        CreatedByName,
    }

    pub struct Filter<'a> {
        pub value: &'a str,
        pub column: Column,
    }
}
