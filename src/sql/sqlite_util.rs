use sqlx::{sqlite::SqliteConnection, Connection};
use std::fs::File;
use std::path::Path;
use crate::data::SQL_FILE;

pub(crate) async fn sql_connect() -> Result<SqliteConnection, String> {
    if !Path::new(SQL_FILE).exists() {
        match File::create(SQL_FILE) {
            Ok(_) => {}
            Err(err) => return Err(err.to_string())
        }
    }

    return match SqliteConnection::connect(SQL_FILE).await {
        Ok(conn) => Ok(conn),
        Err(err) => Err(err.to_string())
    };
}