use axum::http::HeaderMap;
use axum::Json;
use crate::claims::{claims_get_user};
use crate::sql::good;
use crate::sql::good::Good;
use crate::sql::sqlite_util::sql_connect;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct UpdateGoodResult {
    errmsg: String,
    errcode: i8,
}

pub async fn update_good(headers: HeaderMap, Json(good): Json<Good>) -> Json<UpdateGoodResult> {
    let update_good_result = if let Some(mut conn) = sql_connect().await {
        let user = claims_get_user(headers, &mut conn).await;
        match user {
            Ok(_) => {
                match good::update_good(&mut conn, good).await {
                    Ok(_) => {
                        create_update_good_result(
                            String::from(""),
                            0,
                        )
                    }
                    Err(err) => {
                        create_update_good_result(
                            err.to_string(),
                            1,
                        )
                    }
                }
            }
            Err(errmsg) => {
                create_update_good_result(
                    errmsg,
                    2,
                )
            }
        }
    } else {
        create_update_good_result(
            String::from("服务器 sql 连接出现问题"),
            4,
        )
    };

    return Json(update_good_result);
}

fn create_update_good_result(err_msg: String, errcode: i8) -> UpdateGoodResult {
    let errmsg = err_msg.to_string();
    return UpdateGoodResult {
        errmsg,
        errcode,
    };
}
