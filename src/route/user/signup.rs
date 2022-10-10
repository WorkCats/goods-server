use sqlx::SqliteConnection;
use crate::sql::user::{insert_user, User};
use crate::sql::sqlite_util::sql_connect;

pub async fn signup() {
    let user = User {
        is_administrator: true,
        username: String::from("石锅拌饭"),
        password: String::from("xyz6123012002"),
    };
    let connect: Option<SqliteConnection> = sql_connect().await;
    if let Some(mut conn) = connect {
        //调用方法
        match insert_user(&mut conn, user).await {
            Ok(_) => {}
            Err(err) => {}
        };
    }
}