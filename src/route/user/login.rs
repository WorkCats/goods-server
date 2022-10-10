use std::time::{SystemTime, UNIX_EPOCH};
use axum::Json;
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use crate::sql::user::{get_user};

use crate::claims::Claims;
use crate::data::DECODING_KEY;
use crate::sql::sqlite_util::sql_connect;


impl Clone for Claims {
    fn clone(&self) -> Claims {
        Claims {
            username: (self.username).parse().unwrap(),
            password: (*self.password).parse().unwrap(),
            auto_login: self.auto_login
        }
    }
}

#[derive(Serialize, Deserialize)]
struct UserResult {
    token: String,
    errmsg: String,
    errcode: i8
}


pub async fn login(Json(login_user): Json<Claims>) -> String {
    let json: Option<String>;

    json = if let Some(mut conn) = sql_connect().await {
        // clone 对应用户
        let clone_user = login_user.clone();
        if let Some(user) = get_user(&mut conn, login_user.username).await {
            // 判断账号密码
            if login_user.password == user.password {
                let claims = clone_user;
                let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(DECODING_KEY.as_ref()));

                match token {
                    Ok(string) =>
                        create_user_result(string, "", 0),

                    Err(err) => {
                        println!("token create :{:?}", err);
                        create_user_result("".to_string(), "狐雾气出现问题了", 3)
                    }
                }
            } else {
                create_user_result("".to_string(), "账号或者密码填写出现问题", 1)
            }
        } else {
            create_user_result("".to_string(), "当前用户名不存在", 2)
        }
    } else {
        create_user_result("".to_string(), "狐雾气出现问题了", 3)
    };


    return match json {
        None => {
            "{token = \"\", errmsg = \"解析出现问题\", errcode =\"5\"}".to_string()
        }
        Some(json) => {
            json
        }
    };
}


fn create_user_result(token: String, err_msg: &str, errcode: i8) -> Option<String> {
    let errmsg = err_msg.to_string();
    let json = UserResult {
        token,
        errmsg,
        errcode,
    };
    return serde_json::to_string(&json).ok();
}