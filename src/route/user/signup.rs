use axum::Json;
use serde::{Deserialize, Serialize};
use crate::claims::claims_get_user;
use crate::HeaderMap;
use crate::sql::user::{insert_user, User};
use crate::sql::sqlite_util::sql_connect;


#[derive(Serialize, Deserialize)]
pub struct SignupResult {
    errmsg: String,
    errcode: i8,
}

pub async fn signup(headers: HeaderMap, Json(signup_user): Json<User>) -> Json<SignupResult> {
    return Json(if let Some(mut conn) = sql_connect().await {
        match claims_get_user(headers, &mut conn).await {
            Ok(login_user) => {
                if login_user.is_administrator {
                    match insert_user(&mut conn, signup_user).await {
                        Ok(_) => {
                            create_signup_result(String::from(""), 0)
                        }
                        Err(err) => {
                            create_signup_result(err.to_string(), 1)
                        }
                    }
                } else {
                    create_signup_result(String::from("你不是管理员"), 2)
                }
            }

            Err(err) => {
                create_signup_result(err.to_string(), 3)
            }
        }
    } else {
        create_signup_result(String::from("狐雾气出现问题了"), 4)
    });
}

fn create_signup_result(
    err_msg: String,
    errcode: i8,
) -> SignupResult {
    let errmsg = err_msg.to_string();
    return SignupResult {
        errmsg,
        errcode,
    };
}