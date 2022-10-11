use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use sqlx::Error;
use crate::route::{CLAIMS_ERRCODE, SQL_CONNECT_ERRCODE, SQL_ERRCODE, SUCCESS_CODE, SUCCESS_STR, ADMINISTRATOR_STR, ADMINISTRATOR_ERRCODE};
use crate::sql::user::User;

pub mod signup;
pub mod login;
pub mod autologin;
pub mod del_user;
pub mod search_user;
pub mod get_user_list;
pub mod update_user;

pub const NULL_USER_LIST: Vec<User> = Vec::new();

#[derive(Serialize, Deserialize)]
pub struct UserName {
    username: String,
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
