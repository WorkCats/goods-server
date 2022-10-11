use sqlx::{sqlite::SqliteConnection, Sqlite, FromRow, Error};
use serde::{Deserialize, Serialize};

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

async fn create_user(connect: &mut SqliteConnection) {
    let table = sqlx::query::<Sqlite>("CREATE TABLE IF NOT EXISTS users(username text primary key,password text,is_administrator integer)")
        .execute(connect)
        .await;
    match table {
        Ok(result) => {
            println!("create user: {:?}", result);
        }
        Err(err) => {
            println!("create user err message: {:?}", err);
        }
    }
}

pub async fn insert_user(connect: &mut SqliteConnection, user: User) -> Result<bool, Error> {
    create_user(connect).await;
    let sql = sqlx::query::<Sqlite>("insert into users (username,password,is_administrator) values ( $1,$2,$3 )")
        .bind(user.username)
        .bind(user.password)
        .bind(user.is_administrator)
        .execute(connect).await;
    return match sql {
        Ok(_) => {
            Ok(true)
        }
        Err(err) => {
            Err(err)
        }
    };
}


pub async fn get_user(connect: &mut SqliteConnection, username: String) -> Option<User> {
    return match select_user(connect, username).await {
        Ok(user_list) => {
            if let Some(user) = user_list.get(0) {
                let user = user.clone();
                Some(user)
            } else {
                None
            }
        }
        Err(_) => {
            None
        }
    };
}


async fn select_user(connect: &mut SqliteConnection, username: String) -> Result<Vec<User>, Error> {
    let res = sqlx::query_as::<Sqlite, User>(
        "select * FROM users WHERE username = $1"
    ).bind(username)
        .fetch_all(connect)
        .await;
    return res
}


pub async fn delete_user(connect: &mut SqliteConnection, user_name: String) -> Result<bool, Error> {
    create_user(connect).await;
    let sql = sqlx::query::<Sqlite>("DELETE FROM users WHERE username = $1")
        .bind(user_name)
        .execute(connect).await;
    return match sql {
        Ok(_) => {
            Ok(true)
        }
        Err(err) => {
            Err(err)
        }
    };
}