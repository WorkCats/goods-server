use sqlx::{sqlite::SqliteConnection, Sqlite, FromRow, Error};
use serde::{Deserialize, Serialize};
use crate::data::{DEFAULT_PASSWORD, DEFAULT_USERNAME};

#[derive(Deserialize, Serialize, FromRow)]
pub struct User {
    pub username: String,
    pub password: String,
    pub is_administrator: bool,
}


impl Clone for User {
    fn clone(&self) -> User {
        User {
            username: (self.username).parse().unwrap(),
            password: (*self.password).parse().unwrap(),
            is_administrator: self.is_administrator,
        }
    }
}

async fn create_user(connection: &mut SqliteConnection) {
    let table = sqlx::query::<Sqlite>("CREATE TABLE IF NOT EXISTS users(username text primary key, password text, is_administrator integer)")
        .execute(connection)
        .await;

    match table {
        Ok(_) => {}
        Err(err) => {
            println!("create user err message: {:?}", err);
        }
    }
}

async fn into_default_user(connection: &mut SqliteConnection) {
    let _ = sqlx::query::<Sqlite>("INSERT INTO users (username,password,is_administrator) values ( $1,$2,$3 )")
        .bind(DEFAULT_USERNAME)
        .bind(DEFAULT_PASSWORD)
        .bind(true)
        .execute(connection).await;
}

pub async fn insert_user(connection: &mut SqliteConnection, user: User) -> Result<bool, Error> {
    create_user(connection).await;
    match get_all_user(connection).await {
        Ok(users_list) => if users_list.is_empty() {
            into_default_user(connection).await;
        }
        Err(_) => {}
    }
    let sql = sqlx::query::<Sqlite>("INSERT INTO users (username,password,is_administrator) values ( $1,$2,$3 )")
        .bind(user.username)
        .bind(user.password)
        .bind(user.is_administrator)
        .execute(connection).await;
    return match sql {
        Ok(_) => {
            Ok(true)
        }
        Err(err) => {
            Err(err)
        }
    };
}


pub async fn get_user(connection: &mut SqliteConnection, username: String) -> Result<User, Error> {
    create_user(connection).await;
    into_default_user(connection).await;
    return sqlx::query_as::<Sqlite, User>(
        "SELECT * FROM users WHERE username = $1"
    ).bind(username)
        .fetch_one(connection)
        .await;
}

pub async fn search_user(connection: &mut SqliteConnection, username: String) -> Result<Vec<User>, Error> {
    let username = "%".to_owned() + username.as_str() + "%";
    let res = sqlx::query_as::<Sqlite, User>(
        "SELECT * FROM users WHERE username LIKE $1"
    ).bind(username)
        .fetch_all(connection)
        .await;
    return res;
}

pub async fn get_all_user(connection: &mut SqliteConnection) -> Result<Vec<User>, Error> {
    create_user(connection).await;
    let sql = sqlx::query_as::<Sqlite, User>("SELECT * FROM users")
        .fetch_all(connection).await;
    return sql;
}

pub async fn update_user(connection: &mut SqliteConnection, user: User) -> Result<bool, Error> {
    create_user(connection).await;
    let sql = sqlx::query::<Sqlite>("UPDATE users SET is_administrator = $3, password = $2 WHERE username = $1;")
        .bind(user.username)
        .bind(user.password)
        .bind(user.is_administrator)
        .execute(connection).await;
    return match sql {
        Ok(_) => {
            Ok(true)
        }
        Err(err) => {
            Err(err)
        }
    };
}

pub async fn delete_user(connection: &mut SqliteConnection, user_name: String) -> Result<bool, Error> {
    create_user(connection).await;
    let sql = sqlx::query::<Sqlite>("DELETE FROM users WHERE username = $1")
        .bind(user_name)
        .execute(connection).await;
    return match sql {
        Ok(_) => {
            Ok(true)
        }
        Err(err) => {
            Err(err)
        }
    };
}