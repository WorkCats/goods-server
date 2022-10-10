use axum::http::HeaderMap;
use crate::claims::{claims_get_user};
use crate::sql::good::{get_all_good, Good};
use crate::sql::sqlite_util::sql_connect;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct GoodList {
    good_list: Vec<Good>,
    errmsg: String,
    errcode: i8,
}

const NULL_LIST: Vec<Good> = Vec::new();

pub(crate) async fn get_good_list(headers: HeaderMap) -> String {
    let json: Option<String>;
    json = if let Some(mut conn) = sql_connect().await {
        let user = claims_get_user(headers, &mut conn).await;
        match user {
            Ok(_) => {
                let good_list = get_all_good(&mut conn).await;
                match good_list {
                    Some(good_list) => create_good_list_result(good_list, String::from(""), 0),
                    None => create_good_list_result(NULL_LIST, String::from("狐雾气获取货物列表出现的小问题"), 3)
                }
            }
            Err(errmsg) => {
                create_good_list_result(NULL_LIST, errmsg, 2)
            }
        }

    } else {
        create_good_list_result(NULL_LIST, String::from("狐雾气 SQLite 出现问题"), 1)
    };

    return match json {
        None => {
            "{token = \"[]\", errmsg = \"解析出现问题\", errcode =\"4\"}".to_string()
        }
        Some(json) => {
            json
        }
    };
}

fn create_good_list_result(good_list: Vec<Good>, err_msg: String, errcode: i8) -> Option<String> {
    let errmsg = err_msg.to_string();
    let json = GoodList {
        good_list,
        errmsg,
        errcode,
    };
    return serde_json::to_string(&json).ok();
}