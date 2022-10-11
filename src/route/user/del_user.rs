use axum::Json;
use serde::{Deserialize, Serialize};
use crate::claims::claims_get_user;
use crate::HeaderMap;
use crate::sql::user::{delete_user};
use crate::sql::sqlite_util::sql_connect;

#[derive(Serialize, Deserialize)]
pub struct UserName {
    username: String,
}

#[derive(Serialize, Deserialize)]
pub struct DelUserResult {
    errmsg: String,
    errcode: i8,
}

pub async fn del_user(headers: HeaderMap, Json(del_user): Json<UserName>) -> Json<DelUserResult> {
    return Json(
        if let Some(mut conn) = sql_connect().await {
            match claims_get_user(headers, &mut conn).await {
                Ok(login_user) => {
                    if login_user.is_administrator {
                        match delete_user(&mut conn, del_user.username).await {
                            Ok(_) => {
                                create_del_user_result(String::from(""), 0)
                            }
                            Err(err) => {
                                create_del_user_result(err.to_string(), 1)
                            }
                        }
                    } else {
                        create_del_user_result(String::from("你不是管理员"), 2)
                    }
                }

                Err(err) => {
                    create_del_user_result(err.to_string(), 3)
                }
            }
        } else {
            create_del_user_result(String::from("狐雾气出现问题了"), 4)
        });
}

fn create_del_user_result(
    err_msg: String,
    errcode: i8,
) -> DelUserResult {
    let errmsg = err_msg.to_string();
    return DelUserResult {
        errmsg,
        errcode,
    };
}