#[derive(Deserialize, Serialize, sqlx::FromRow, Clone)]
pub struct Log {
    time: String,
    user_name: String,
    event: String,
}

async fn create_log(connection: &mut SqliteConnection) {
    let table = sqlx::query::<Sqlite>("CREATE TABLE IF NOT EXISTS logs(time text primary key,event text, user_name text)").execute(connection).await;
    match table {
        Ok(_) => {}
        Err(err) => {
            println!("create goods err message: {:?}", err);
        }
    }
}

async fn insert_log(connection: &mut SqliteConnection, log: Log) -> Result<bool, Error> {
    create_log(connection).await;
    let sql = sqlx::query::<Sqlite>("INSERT INTO logs(id,name, event, user_name) values ($1, $2, $3)")
        .bind(log.time)
        .bind(log.event)
        .bind(log.user_name)
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