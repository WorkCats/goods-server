use axum::Json;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

use crate::sql::{
    sqlite_util::sql_connect,
    user::get_user,
};
use crate::route::{SUCCESS_CODE, SUCCESS_STR, SQL_CONNECT_ERRCODE};
use crate::route::user::{NULL_TOKEN, token_from_user, UserResult};

#[derive(Serialize, Deserialize)]
pub struct LoginUser {
    // 是否是自动登录
    pub auto_login: bool,
    pub username: String,
    pub password: String,
}

impl Clone for LoginUser {
    fn clone(&self) -> LoginUser {
        LoginUser {
            username: (self.username).parse().unwrap(),
            password: (*self.password).parse().unwrap(),
            auto_login: self.auto_login,
        }
    }
}

fn create_user_success_result(token: String) -> UserResult {
    return UserResult {
        token,
        errmsg: SUCCESS_STR.to_string(),
        errcode: SUCCESS_CODE,
    };
}

// 在生成 TOKEN 时候出现问题
pub static TOKEN_ERRCODE: i8 = 5;

fn create_user_result_token_err(errmsg: String) -> UserResult {
    return UserResult {
        token: NULL_TOKEN.to_string(),
        errmsg,
        errcode: TOKEN_ERRCODE,
    };
}

// 密码出问题出现问题
static PASSWORD_ERRCODE: i8 = 6;
static PASSWORD_STR: &'static str = "当前账户或者密码出现问题了，喵";
// 当前用户不存在
static USERNAME_ERRCODE: i8 = 7;
static USERNAME_STR: &'static str = "当前用户名不存在";

pub(crate) fn create_user_result_sql_connect_err(errmsg: String) -> UserResult {
    return UserResult {
        token: NULL_TOKEN.to_string(),
        errmsg,
        errcode: SQL_CONNECT_ERRCODE,
    };
}
lazy_static! {
    static ref USER_RESULT_PASSWORD_ERR:UserResult = UserResult {
        token: NULL_TOKEN.to_string(),
        errmsg: PASSWORD_STR.to_string(),
        errcode: PASSWORD_ERRCODE
    };

    static ref USER_RESULT_USERNAME_ERR:UserResult = UserResult {
        token: NULL_TOKEN.to_string(),
        errmsg: USERNAME_STR.to_string(),
        errcode:USERNAME_ERRCODE
    };
}

pub(in crate::route) async fn login(Json(login_user): Json<LoginUser>) -> Json<UserResult> {
    let clone_user = login_user.clone();
    return Json(match sql_connect().await {
        // clone 对应用户

        Ok(mut conn) => match get_user(&mut conn, login_user.username).await {
            // 判断账号密码
            Ok(user) => if login_user.password == user.password {

                match token_from_user(clone_user) {
                    Ok(string) => create_user_success_result(string),

                    Err(err) => create_user_result_token_err(err.to_string())
                }
            } else {
                USER_RESULT_PASSWORD_ERR.clone()
            }

            Err(_) => USER_RESULT_USERNAME_ERR.clone()
        }
        Err(err) => create_user_result_sql_connect_err(err.to_string())
    });
}
