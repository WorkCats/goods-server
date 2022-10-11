use axum::http::HeaderMap;
use axum::Json;
use crate::claims::{claims_get_user};
use crate::sql::good::{get_all_good, Good};
use crate::sql::sqlite_util::sql_connect;
use serde::{Deserialize, Serialize};
use crate::route::good::NULL_GOOD_LIST;

#[derive(Serialize, Deserialize)]
pub struct GoodList {
    good_list: Vec<Good>,
    errmsg: String,
    errcode: i8,
}


pub async fn get_good_list(headers: HeaderMap) -> Json<GoodList> {
    let good_list = if let Some(mut conn) = sql_connect().await {
        let user = claims_get_user(headers, &mut conn).await;
        match user {
            Ok(_) => {
                let good_list = get_all_good(&mut conn).await;
                match good_list {
                    Ok(good_list) => create_good_list_result(good_list, String::from(""), 0),
                    Err(err) => create_good_list_result(NULL_GOOD_LIST, err.to_string(), 3)
                }
            }
            Err(errmsg) => {
                create_good_list_result(NULL_GOOD_LIST, errmsg, 2)
            }
        }

    } else {
        create_good_list_result(NULL_GOOD_LIST, String::from("狐雾气 SQLite 出现问题"), 1)
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