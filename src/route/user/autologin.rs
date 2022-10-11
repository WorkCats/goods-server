use axum::{http::HeaderMap, Json};
use serde::{Deserialize, Serialize};

use crate::claims::{claims_get_autologin, claims_get_user};
use crate::sql::sqlite_util::sql_connect;

#[derive(Serialize, Deserialize)]
pub struct AutologinResult {
    autologin: bool,
    errmsg: String,
    errcode: i8,
}

pub async fn autologin(headers: HeaderMap) -> Json<AutologinResult> {
    let headers_clone = headers.clone();
    return Json(
        if let Some(mut conn) = sql_connect().await {
            match claims_get_user(headers, &mut conn).await {
                Ok(_) => {
                    match claims_get_autologin(headers_clone).await {
                        Ok(is_autologin) => {
                            create_auto_login_result(
                                is_autologin,
                                String::from(""),
                                0,
                            )
                        }
                        Err(err) => {
                            create_auto_login_result(
                                false,
                                err.to_string(),
                                0,
                            )
                        }
                    }
                }
                Err(err) => {
                    create_auto_login_result(
                        false,
                        err.to_string(),
                        1,
                    )
                }
            }
        } else {
            create_auto_login_result(
                false,
                String::from("服务器 sql 出现问题"),
                1,
            )
        }
    );
}

fn create_auto_login_result(
    autologin: bool,
    err_msg: String,
    errcode: i8,
) -> AutologinResult {
    let errmsg = err_msg.to_string();
    return AutologinResult {
        autologin,
        errmsg,
        errcode,
    };
}