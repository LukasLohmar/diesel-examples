use diesel::prelude::*;
use diesel_migrations::{embed_migrations, EmbeddedMigrations};

mod schema;
mod models;

pub const SQLITE_MIGRATIONS: EmbeddedMigrations = embed_migrations!();

pub fn establish_connection(database_url: &str) -> SqliteConnection {
    SqliteConnection::establish(&database_url).unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

#[cfg(test)]
mod test {
    use rstest::rstest;
    use tokio_test::assert_ok;
    use diesel_migrations::MigrationHarness;

    use crate::establish_connection;
    use crate::models::user::User;

    #[rstest]
    #[case(String::from("sqlite://test.db"))]
    #[tokio::test]
    async fn test_jsonb_types(#[case] url: String) {
        use diesel::prelude::*;
        use crate::models::user::{User, filter::{Filter, Column as FilterColumn}};

        use crate::SQLITE_MIGRATIONS;

        let connection = &mut establish_connection(&url);
        
        let migration = connection.run_pending_migrations(SQLITE_MIGRATIONS);

        assert_ok!(migration);

        let filter = Filter {
            value: "Dennis",
            column: FilterColumn::ContactName,
        };

        let results = User::execute_filter(connection, &filter);

        match results {
            Ok(results) => {
                for result in results.into_iter() {
                    println!("{:?}", result);
                }
            },
            Err(err) => println!("{:?}", err),
        }
    }
}
