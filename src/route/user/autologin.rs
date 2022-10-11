use axum::{http::HeaderMap, Json};
use serde::{Deserialize, Serialize};

use crate::claims::{claims_get_autologin, claims_get_user};
use crate::route::{
    CLAIMS_ERRCODE,
    SUCCESS_CODE,
    SUCCESS_STR,
    SQL_CONNECT_ERRCODE,
};
use crate::sql::sqlite_util::sql_connect;

#[derive(Serialize, Deserialize)]
pub struct AutologinResult {
    autologin: bool,
    errmsg: String,
    errcode: i8,
}

impl Clone for AutologinResult {
    fn clone(&self) -> AutologinResult {
        return AutologinResult {
            autologin: self.autologin,
            errmsg: (self.errmsg).parse().unwrap(),
            errcode: self.errcode,
        };
    }
}

fn create_auto_login_success_result(
    autologin: bool,
) -> AutologinResult {
    return AutologinResult {
        autologin,
        errmsg: SUCCESS_STR.to_string(),
        errcode: SUCCESS_CODE,
    };
}

fn create_auto_login_result_claims_err(errmsg: String) -> AutologinResult {
    return AutologinResult {
        autologin: false,
        errmsg,
        errcode: CLAIMS_ERRCODE,
    };
}

fn create_auto_login_result_sql_connect_err(errmsg: String) -> AutologinResult {
    return AutologinResult {
        autologin: false,
        errmsg,
        errcode: SQL_CONNECT_ERRCODE,
    };
}

pub async fn autologin(headers: HeaderMap) -> Json<AutologinResult> {
    let headers_clone = headers.clone();
    return Json(
        match sql_connect().await {
            Ok(mut conn) => match claims_get_user(headers, &mut conn).await {
                Ok(_) => match claims_get_autologin(headers_clone).await {
                    Ok(is_autologin) => create_auto_login_success_result(is_autologin),

                    Err(err) => create_auto_login_result_claims_err(err.to_string())
                }

                Err(err) => create_auto_login_result_claims_err(err)
            }
            Err(err) => create_auto_login_result_sql_connect_err(err.to_string())
        }
    );
}
