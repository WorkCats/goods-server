use axum::http::HeaderMap;
use axum::Json;
use crate::claims::{claims_get_user};
use crate::sql::sqlite_util::sql_connect;
use serde::{Deserialize, Serialize};
use crate::route::good::NULL_GOOD_LIST;

use crate::sql::good::{Good, select_good};

#[derive(Serialize, Deserialize)]
pub struct GoodName {
    good_name: String
}

#[derive(Serialize, Deserialize)]
pub struct SearchGoodList {
    good_list: Vec<Good>,
    errmsg: String,
    errcode: i8,
}

pub async fn search_good(headers: HeaderMap, Json(good): Json<GoodName>) -> Json<SearchGoodList> {
    let good_list = if let Some(mut conn) = sql_connect().await {
        let user = claims_get_user(headers, &mut conn).await;
        match user {
            Ok(_) => {
                let good_list = select_good(&mut conn, good.good_name).await;
                match good_list {
                    Ok(good_list) => create_search_good_result(good_list, String::from(""), 0),
                    Err(err) => create_search_good_result(NULL_GOOD_LIST, err.to_string(), 3)
                }
            }
            Err(errmsg) => {
                create_search_good_result(NULL_GOOD_LIST, errmsg, 2)
            }
        }
    } else {
        create_search_good_result(NULL_GOOD_LIST, String::from("狐雾气 SQLite 出现问题"), 1)
    };

    return Json(good_list)
}

fn create_search_good_result(good_list: Vec<Good>, err_msg: String, errcode: i8) -> SearchGoodList {
    let errmsg = err_msg.to_string();
    return SearchGoodList {
        good_list,
        errmsg,
        errcode,
    }
}