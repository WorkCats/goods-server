use std::time::{SystemTime, UNIX_EPOCH};
use axum::Json;
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use crate::sql::user::{get_user};

use crate::claims::Claims;
use crate::data::DECODING_KEY;
use crate::sql::sqlite_util::sql_connect;

#[derive(Serialize, Deserialize)]
pub struct LoginUser {
    // 是否是自动登录
    pub auto_login: bool,
    // 可选。标题 (令牌指向的人)
    pub username: String,
    pub password: String
}

impl Clone for LoginUser {
    fn clone(&self) -> LoginUser {
        LoginUser {
            username: (self.username).parse().unwrap(),
            password: (*self.password).parse().unwrap(),
            auto_login: self.auto_login
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct UserResult {
    token: String,
    errmsg: String,
    errcode: i8
}


pub async fn login(Json(login_user): Json<LoginUser>) -> Json<UserResult> {
    let json: UserResult;

    json = if let Some(mut conn) = sql_connect().await {
        // clone 对应用户
        let clone_user = login_user.clone();
        if let Some(user) = get_user(&mut conn, login_user.username).await {
            // 判断账号密码
            if login_user.password == user.password {
                let start = SystemTime::now();
                let since_the_epoch = start
                    .duration_since(UNIX_EPOCH)
                    .expect("Time went backwards");
                let exp = since_the_epoch.as_secs();
                let username = clone_user.username;
                let password = clone_user.password;
                let auto_login = clone_user.auto_login;
                let claims = Claims{
                    exp,
                    username,
                    password,
                    auto_login
                };

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
    return Json(json)
}


fn create_user_result(token: String, err_msg: &str, errcode: i8) -> UserResult {
    let errmsg = err_msg.to_string();
    return UserResult {
        token,
        errmsg,
        errcode,
    }
}