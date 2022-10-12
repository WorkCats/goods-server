use std::time::{SystemTime, UNIX_EPOCH};
use jsonwebtoken::{encode, EncodingKey, Header};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use sqlx::Error;
use crate::claims::Claims;
use crate::data::DECODING_KEY;
use crate::route::{CLAIMS_ERRCODE, SQL_CONNECT_ERRCODE, SQL_ERRCODE, SUCCESS_CODE, SUCCESS_STR, ADMINISTRATOR_STR, ADMINISTRATOR_ERRCODE};
use crate::sql::user::User;
use crate::route::user::login::LoginUser;

pub mod signup;
pub mod login;
pub mod autologin;
pub mod del_user;
pub mod search_user;
pub mod get_user_list;
pub mod update_user;
pub mod get_username_list;
pub mod get_goods;
pub mod is_administrator;

pub const NULL_USER_LIST: Vec<User> = Vec::new();

#[derive(Serialize, Deserialize)]
pub struct UserName {
    username: String,
}

static NULL_TOKEN: &'static str = "";
#[derive(Serialize, Deserialize)]
pub struct UserResult {
    token: String,
    errmsg: String,
    errcode: i8,
}

impl Clone for UserResult {
    fn clone(&self) -> UserResult {
        return UserResult {
            token: (self.token).parse().unwrap(),
            errmsg: (*self.errmsg).parse().unwrap(),
            errcode: self.errcode,
        };
    }
}

#[derive(Serialize, Deserialize)]
pub struct UserListResult {
    user_list: Vec<User>,
    errmsg: String,
    errcode: i8,
}

impl Clone for UserListResult {
    fn clone(&self) -> UserListResult {
        UserListResult {
            user_list: (*self.user_list).to_vec(),
            errmsg: (self.errmsg).parse().unwrap(),
            errcode: self.errcode,
        }
    }
}


fn create_user_list_success_result(user_list: Vec<User>) -> UserListResult {
    return UserListResult {
        user_list,
        errmsg: SUCCESS_STR.to_string(),
        errcode: SUCCESS_CODE,
    };
}

fn create_user_list_result_sql_err(err_msg: Error) -> UserListResult {
    let errmsg = err_msg.to_string();
    return UserListResult {
        user_list: NULL_USER_LIST,
        errmsg,
        errcode: SQL_ERRCODE,
    };
}

fn create_user_list_result_claims_err(errmsg: String) -> UserListResult {
    return UserListResult {
        user_list: NULL_USER_LIST,
        errmsg,
        errcode: CLAIMS_ERRCODE,
    };
}

fn create_user_list_result_sql_connect_err(err_msg: Error) -> UserListResult {
    let errmsg = err_msg.to_string();
    return UserListResult {
        user_list: NULL_USER_LIST,
        errmsg,
        errcode: SQL_CONNECT_ERRCODE,
    };
}

lazy_static! {

    pub static ref USER_LIST_RESULT_ADMINISTRATOR_ERRCODE: UserListResult = UserListResult {
        user_list: NULL_USER_LIST,
        errmsg: ADMINISTRATOR_STR.to_string(),
        errcode: ADMINISTRATOR_ERRCODE
    };

}

fn token_from_user(user: LoginUser) -> Result<String, jsonwebtoken::errors::Error> {
    let start = SystemTime::now();
    let exp = start.duration_since(UNIX_EPOCH).expect("Time went backwards").as_secs() + 7 * 60 * 60 * 24;
    let username = user.username;
    let password = user.password;
    let auto_login = user.auto_login;
    return match encode(
        &Header::default(),
        &Claims {
            exp,
            username,
            password,
            auto_login,
        },
        &EncodingKey::from_secret(DECODING_KEY.as_ref()),
    ) {
        Ok(token) =>{
            Ok(token)
        }
        Err(err)=>{
            Err(err)
        }
    };
}