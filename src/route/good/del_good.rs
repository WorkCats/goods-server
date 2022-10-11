use axum::http::HeaderMap;
use axum::Json;
use crate::claims::{claims_get_user};
use crate::sql::good;
use crate::sql::sqlite_util::sql_connect;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct GoodName {
    good_name: String,
}

#[derive(Serialize, Deserialize)]
pub struct DelGoodResult {
    errmsg: String,
    errcode: i8,
}

pub async fn del_good(headers: HeaderMap, Json(good): Json<GoodName>) -> Json<DelGoodResult> {
    return Json(if let Some(mut conn) = sql_connect().await {
        let user = claims_get_user(headers, &mut conn).await;
        match user {
            Ok(_) => {
                match good::delete_good(&mut conn, good.good_name).await {
                    Ok(_) => {
                        create_del_good_result(
                            String::from(""),
                            0,
                        )
                    }
                    Err(err) => {
                        create_del_good_result(
                            err.to_string(),
                            1,
                        )
                    }
                }
            }
            Err(errmsg) => {
                create_del_good_result(
                    errmsg,
                    2,
                )
            }
        }
    } else {
        create_del_good_result(
            String::from("服务器 sql 连接出现问题"),
            4,
        )
    });
}

fn create_del_good_result(err_msg: String, errcode: i8) -> DelGoodResult {
    let errmsg = err_msg.to_string();
    return DelGoodResult {
        errmsg,
        errcode,
    };
}
