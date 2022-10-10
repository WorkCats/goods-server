use sqlx::{sqlite::SqliteConnection, Connection};
use crate::data::SQL_FILE;

pub(crate) async fn sql_connect() -> Option<SqliteConnection> {
    let connect = SqliteConnection::connect(SQL_FILE).await;
    match connect {
        Ok(conn) => {
            Some(conn)
        }
        Err(err) => {
            println!("sql connect err message: {:?}", err);
            None
        }
    }
}