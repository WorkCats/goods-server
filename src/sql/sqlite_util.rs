use sqlx::{sqlite::SqliteConnection, Connection, Error};
use crate::data::SQL_FILE;

pub(crate) async fn sql_connect() -> Result<SqliteConnection, Error> {
    match SqliteConnection::connect(SQL_FILE).await {
        Ok(conn) => {
            Ok(conn)
        }
        Err(err) => {
            Err(err)
        }
    }
}