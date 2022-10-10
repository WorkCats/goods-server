use crate::claims::claims_get_autologin;
use crate::HeaderMap;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct autologinResult {
    autologin: bool,
    errmsg: String,
    errcode: i8,
}
pub async fn autologin(headers: HeaderMap) -> String {
    let json: Option<String>;
    json = match claims_get_autologin(headers).await {
        Ok(is_autologin) => {
            create_auto_login_result(
                is_autologin,
                String::from(""),
                0
            )
        }
        Err(err)=>{
            create_auto_login_result(
                false,
                err.to_string(),
                1
            )
        }
    };
    return match json {
        None => {
            "{autologin=\"false\",errmsg = \"解析出现问题\", errcode =\"3\"}".to_string()
        }
        Some(json) => {
            json
        }
    };

}

fn create_auto_login_result(autologin: bool, err_msg: String, errcode: i8) -> Option<String> {
    let errmsg = err_msg.to_string();
    let json = autologinResult {
        autologin,
        errmsg,
        errcode,
    };
    return serde_json::to_string(&json).ok();
}