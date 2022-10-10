use axum::http::HeaderMap;
use crate::claims::{claims_get_user};
use crate::sql::good;
use crate::sql::sqlite_util::sql_connect;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct delGoodResult {
    errmsg: String,
    errcode: i8,
}

pub async fn del_good(headers: HeaderMap, good_name: String) -> String {
    let json = if let Some(mut conn) = sql_connect().await {
        let user = claims_get_user(headers, &mut conn).await;
        match user {
            Ok(_) => {
                match good::delete_good(&mut conn, good_name).await {
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
    };

    return match json {
        None => {
            "{errmsg = \"解析出现问题\", errcode =\"3\"}".to_string()
        }
        Some(json) => {
            json
        }
    };
}

fn create_del_good_result(err_msg: String, errcode: i8) -> Option<String> {
    let errmsg = err_msg.to_string();
    let json = delGoodResult {
        errmsg,
        errcode,
    };
    return serde_json::to_string(&json).ok();
}
