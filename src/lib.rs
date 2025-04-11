use diesel::prelude::*;
use diesel_migrations::{EmbeddedMigrations, MigrationHarness, embed_migrations};

mod models;
mod schema;

pub const SQLITE_MIGRATIONS: EmbeddedMigrations = embed_migrations!();

pub fn establish_connection<'a>(database_url: &str) -> SqliteConnection {
    let mut connection = SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url));

    {
        let migration_connection = &mut connection;
        migration_connection
            .run_pending_migrations(SQLITE_MIGRATIONS)
            .unwrap();
    }

    connection
}

#[cfg(test)]
mod test {
    use diesel_migrations::MigrationHarness;
    use rstest::rstest;

    use crate::establish_connection;
    use crate::models::user::User;
    use crate::models::user::filter::{Column as UserFilterColumn, Filter as UserFilter};

    #[rstest]
    #[case(
        String::from(":memory:"),
        vec![
            (
                String::from("filter:   CreatedById = 1"),
                UserFilter {
                    value: "1",
                    column: UserFilterColumn::CreatedById,
                    }
            ),
            (
                String::from("filter:   ContactName = 'Dennis'"),
                UserFilter {
                    value: "Dennis",
                    column: UserFilterColumn::ContactName,
                }
            ),
            (
                String::from("filter:   ContactName = 'Lukas'"),
                UserFilter {
                    value: "Lukas",
                    column: UserFilterColumn::ContactName,
                }
            ),
            (
                String::from("filter:   ContactName = 'Admin'"),
                UserFilter {
                    value: "Admin",
                    column: UserFilterColumn::CreatedByName,
                }
            ),
            (
                String::from("filter:   CreatedByName = 'Lukas-User'"),
                UserFilter {
                    value: "Lukas-User",
                    column: UserFilterColumn::CreatedByName,
                }
            )
        ]
    )]
    #[tokio::test]
    async fn alias_with_boxable_expression(
        #[case] url: String,
        #[case] filter: Vec<(String, UserFilter<'_>)>,
    ) {
        let connection = &mut establish_connection(&url);

        filter.iter().for_each(|(description, filter)| {
            let results = User::execute_filter(connection, &filter);

            println!("\n{}\n", description);

            match results {
                Ok(results) => {
                    for result in results.into_iter() {
                        println!("{:?}", result);
                    }
                }
                Err(err) => println!("{:?}", err),
            }
        });

        println!("\n");
    }
}
