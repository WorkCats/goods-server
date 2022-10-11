use crate::claims::claims_get_autologin;
use crate::HeaderMap;
use serde::{Deserialize, Serialize};
use axum::Json;

#[derive(Serialize, Deserialize)]
pub struct AutologinResult {
    autologin: bool,
    errmsg: String,
    errcode: i8,
}

pub async fn autologin(headers: HeaderMap) -> Json<AutologinResult> {
    return Json(match claims_get_autologin(headers).await {
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
                1,
            )
        }
    });
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