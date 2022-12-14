use jsonwebtoken::{Algorithm, decode, DecodingKey, Validation};
use serde::{Serialize, Deserialize};
use axum::http::header::AUTHORIZATION;
use axum::http::HeaderMap;
use jsonwebtoken::errors::Error;
use sqlx::SqliteConnection;

use crate::data::DECODING_KEY;
use crate::sql::user::{get_user, User};

/// 我们的声言结构型, 需要由`Serialize` 或 `Deserialize` 派生
#[derive(Serialize, Deserialize)]
pub struct Claims {
    // 必需品
    pub exp: u64,
    // 是否是自动登录
    pub auto_login: bool,
    // 可选。标题 (令牌指向的人)
    pub username: String,
    pub password: String,
}

/// 从请求中提取 token
/// `headers` 请求头
fn get_cookies(headers: HeaderMap) -> String {
    return headers.get(AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
        .map(|v| v.to_string())
        .unwrap_or("".to_string());
}

/// 判断是否当前 token 是否是自动登录
/// `headers` 请求头
pub async fn claims_get_autologin(headers: HeaderMap) -> Result<bool, Error> {
    let cookies = get_cookies(headers);
    let claims = decode::<Claims>(
        &cookies,
        &DecodingKey::from_secret(DECODING_KEY.as_ref()),
        &Validation::new(Algorithm::HS256),
    );
    match claims {
        Ok(claims) => {
            Ok(claims.claims.auto_login)
        }
        Err(err) => {
            Err(err)
        }
    }
}

/// 获取对应用户的请求头
/// `headers` 请求头
/// `connection` 为 SqliteConnection
pub async fn claims_get_user(headers: HeaderMap, connection: &mut SqliteConnection) -> Result<User, String> {
    let cookies = get_cookies(headers);

    let claims = decode::<Claims>(
        &cookies,
        &DecodingKey::from_secret(DECODING_KEY.as_ref()),
        &Validation::new(Algorithm::HS256),
    );
    match claims {
        Ok(claims) => {
            match get_user(connection, claims.claims.username).await {
                Ok(user) => if user.password == claims.claims.password {
                    Ok(user)
                } else {
                    Err(String::from("账号密码不符合"))
                }
                Err(_) => Err(String::from("不存在当前用户"))
            }
        }

        Err(e) => {
            Err(e.to_string())
        }
    }
}