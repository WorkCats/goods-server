use sqlx::{sqlite::SqliteConnection, Sqlite, Error};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, sqlx::FromRow, Clone)]
pub struct Good {
    pub id: String,
    pub name: String,
    pub size: i64,
    pub user_name: String,
}

async fn create_good(connection: &mut SqliteConnection) {
    let table = sqlx::query::<Sqlite>("CREATE TABLE IF NOT EXISTS goods(id text primary key,name text,size integer, user_name text)").execute(connection).await;
    match table {
        Ok(_) => {}
        Err(err) => {
            println!("create goods err message: {:?}", err);
        }
    }
}

pub async fn insert_good(connection: &mut SqliteConnection, good: Good) -> Result<bool, Error> {
    create_good(connection).await;
    let sql = sqlx::query::<Sqlite>("INSERT INTO goods (id,name,size,user_name) values ( $1,$2,$3,$4 )")
        .bind(good.id)
        .bind(good.name)
        .bind(good.size)
        .bind(good.user_name)
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

pub async fn update_good(connection: &mut SqliteConnection, good: Good) -> Result<bool, Error> {
    create_good(connection).await;
    let sql = sqlx::query::<Sqlite>("UPDATE goods SET name = $2, size = $3, user_name = $4 WHERE ID = $1;")
        .bind(good.id)
        .bind(good.name)
        .bind(good.size)
        .bind(good.user_name)
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

pub async fn select_good(connection: &mut SqliteConnection, name: String) -> Result<Vec<Good>, Error> {
    create_good(connection).await;
    let name = "%".to_owned() + name.as_str() + "%";
    let sql =
        sqlx::query_as::<Sqlite, Good>("SELECT * FROM goods WHERE name LIKE $1")
            .bind(name)
            .fetch_all(connection).await;
    return sql;
}

pub async fn select_good_with_user(connection: &mut SqliteConnection, username: String) -> Result<Vec<Good>, Error> {
    create_good(connection).await;
    let sql =
        sqlx::query_as::<Sqlite, Good>("SELECT * FROM goods WHERE user_name = $1")
            .bind(username)
            .fetch_all(connection).await;
    return sql;
}

pub async fn get_all_good(connection: &mut SqliteConnection) -> Result<Vec<Good>, Error> {
    create_good(connection).await;
    let sql = sqlx::query_as::<Sqlite, Good>("SELECT * FROM goods")
        .fetch_all(connection).await;
    return sql;
}

pub async fn delete_good(connection: &mut SqliteConnection, id: String) -> Result<bool, Error> {
    create_good(connection).await;
    let sql = sqlx::query::<Sqlite>("DELETE FROM goods WHERE ID = $1")
        .bind(id)
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
