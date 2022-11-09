use axum::{http::HeaderMap, Json};
use serde::{Deserialize, Serialize};
use sqlx::Error;
use crate::claims::{claims_get_user};
use crate::route::{CLAIMS_ERRCODE, SQL_CONNECT_ERRCODE, SQL_ERRCODE, SUCCESS_CODE, SUCCESS_STR};

use crate::sql::sqlite_util::sql_connection;
use crate::sql::user::get_all_user;

pub const NULL_USERNAME_LIST: Vec<String> = Vec::new();

#[derive(Serialize, Deserialize)]
pub struct UserNameListResult {
    username_list: Vec<String>,
    errmsg: String,
    errcode: i8,
}

fn create_user_name_list_success_result(username_list: Vec<String>) -> UserNameListResult {
    return UserNameListResult {
        username_list,
        errmsg: SUCCESS_STR.to_string(),
        errcode: SUCCESS_CODE,
    };
}

fn create_user_name_list_result_sql_err(err_msg: Error) -> UserNameListResult {
    let errmsg = err_msg.to_string();
    return UserNameListResult {
        username_list: NULL_USERNAME_LIST,
        errmsg,
        errcode: SQL_ERRCODE,
    };
}

fn create_user_name_list_result_claims_err(errmsg: String) -> UserNameListResult {
    return UserNameListResult {
        username_list: NULL_USERNAME_LIST,
        errmsg,
        errcode: CLAIMS_ERRCODE,
    };
}

fn create_user_name_list_result_sql_connect_err(errmsg: String) -> UserNameListResult {
    return UserNameListResult {
        username_list: NULL_USERNAME_LIST,
        errmsg,
        errcode: SQL_CONNECT_ERRCODE,
    };
}

pub(in crate::route) async fn get_username_list(headers: HeaderMap) -> Json<UserNameListResult> {
    return Json(match sql_connection().await {
        Ok(mut conn) => match claims_get_user(headers, &mut conn).await {
            Ok(_) => match get_all_user(&mut conn).await {
                Ok(user_list) => {
                    let mut username_list: Vec<String> = Vec::new();
                    for user in user_list{
                        username_list.push(user.username)
                    }
                    create_user_name_list_success_result(username_list)
                },
                Err(err) => create_user_name_list_result_sql_err(err)
            }
            Err(errmsg) => create_user_name_list_result_claims_err(errmsg)
        }
        Err(err) => create_user_name_list_result_sql_connect_err(err)
    });
}