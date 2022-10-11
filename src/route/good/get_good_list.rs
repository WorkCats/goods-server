use axum::http::HeaderMap;
use axum::Json;
use crate::claims::{claims_get_user};
use crate::sql::good::{get_all_good, Good};
use crate::sql::sqlite_util::sql_connect;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct GoodList {
    good_list: Vec<Good>,
    errmsg: String,
    errcode: i8,
}

const NULL_LIST: Vec<Good> = Vec::new();

pub async fn get_good_list(headers: HeaderMap) -> Json<GoodList> {
    let good_list = if let Some(mut conn) = sql_connect().await {
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

    return Json(good_list)
}

fn create_good_list_result(good_list: Vec<Good>, err_msg: String, errcode: i8) -> GoodList {
    let errmsg = err_msg.to_string();
    return GoodList {
        good_list,
        errmsg,
        errcode,
    }
}