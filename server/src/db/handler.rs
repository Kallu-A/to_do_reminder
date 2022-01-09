use diesel::connection::SimpleConnection;
use diesel::{Connection, SqliteConnection};
use dotenv::dotenv;
use std::env;

/// Allow a connection to the database quick, easy and multi-threading
pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let con = SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url));
    // Add busy_timeout to handle multi thread request
    if con.batch_execute("PRAGMA busy_timeout = 5000;").is_err() {
        panic!("Error while setting busy_timeout !");
    };
    con
}

#[cfg(test)]
mod tests {
    use crate::db::handler::establish_connection;

    // Make sur that the establish_connection is working
    #[test]
    fn establish() {
        establish_connection();
    }
}
