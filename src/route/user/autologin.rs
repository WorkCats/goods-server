use axum::{http::HeaderMap, Json};
use lazy_static::lazy_static;

use crate::claims::{claims_get_autologin, claims_get_user};
use crate::route::{
    CLAIMS_ERRCODE,
    SUCCESS_CODE,
    SUCCESS_STR
};
use crate::route::user::{NULL_TOKEN, token_from_user, UserResult};
use crate::route::user::login::{create_user_result_sql_connect_err, LoginUser};
use crate::sql::sqlite_util::sql_connect;

fn create_auto_login_success_result(
    token: String,
) -> UserResult {
    return UserResult {
        token,
        errmsg: SUCCESS_STR.to_string(),
        errcode: SUCCESS_CODE,
    };
}

fn create_auto_login_result_claims_err(errmsg: String) -> UserResult {
    return UserResult {
        token: NULL_TOKEN.to_string(),
        errmsg,
        errcode: CLAIMS_ERRCODE,
    };
}

// 并非持久登录
static AUTO_LOGIN_ERRCODE: i8 = 8;
static AUTO_LOGIN_STR: &'static str = "并非永久登录账户";
lazy_static! {

    static ref USER_RESULT_AUTO_LOGIN_ERR:UserResult = UserResult {
        token: NULL_TOKEN.to_string(),
        errmsg: AUTO_LOGIN_STR.to_string(),
        errcode:AUTO_LOGIN_ERRCODE
    };
}
pub async fn autologin(headers: HeaderMap) -> Json<UserResult> {
    let headers_clone = headers.clone();
    return Json(
        match sql_connect().await {
            Ok(mut conn) => match claims_get_user(headers, &mut conn).await {
                Ok(user) => match claims_get_autologin(headers_clone).await {
                    Ok(auto_login) => {
                        if auto_login {
                            let login_user = LoginUser {
                                password: user.password,
                                username: user.username,
                                auto_login,
                            };
                            match token_from_user(login_user) {
                                Ok(token) => create_auto_login_success_result(token),
                                Err(err) => create_auto_login_result_claims_err(err.to_string())
                            }
                        } else {
                            USER_RESULT_AUTO_LOGIN_ERR.clone()
                        }
                    }

                    Err(err) => create_auto_login_result_claims_err(err.to_string())
                }

                Err(errmsg) => create_auto_login_result_claims_err(errmsg)
            }
            Err(err) => create_user_result_sql_connect_err(err.to_string())
        }
    );
}
