mod user;
mod good;

use std::string::ToString;

use lazy_static::lazy_static;
use sqlx::Error;
use axum::{routing::post, Router};
use serde::{Deserialize, Serialize};

use crate::route::good::{
    add_good::add_good,
    get_good_list::get_good_list,
    del_good::del_good,
    update_good::update_good,
    search_good::search_good,
};
use crate::route::user::{
    login::login,
    signup::signup,
    autologin::autologin,
    del_user::del_user,
};
use crate::route::user::get_user_list::get_user_list;
use crate::route::user::search_user::search_user;
use crate::route::user::update_user::update_user;

/**
 * TextResult 是在客户端与服务端交互，不需要返回数据时使用的结构体
 * 告诉客户端本次结果如果
 */
#[derive(Serialize, Deserialize)]
pub struct TextResult {
    errmsg: String,
    errcode: i8,
}

impl Clone for TextResult {
    fn clone(&self) -> TextResult {
        TextResult {
            errmsg: (self.errmsg).parse().unwrap(),
            errcode: self.errcode,
        }
    }
}
// 成功时 CODE
pub static SUCCESS_CODE: i8 = 0;
static SUCCESS_STR: &'static str = "";

// 在 SQL 语句出错时所需错误
pub static SQL_ERRCODE: i8 = 1;

fn create_text_result_sql_err(err_msg: Error) -> TextResult {
    let errmsg = err_msg.to_string();
    return TextResult {
        errmsg,
        errcode: SQL_ERRCODE,
    };
}
// 在 CLAIMS 检测时时所需错误
pub static CLAIMS_ERRCODE: i8 = 2;

fn create_text_result_claims_err(errmsg: String) -> TextResult {
    return TextResult {
        errmsg,
        errcode: CLAIMS_ERRCODE,
    };
}

// 在 SQL 连接出现问题时所需错误
pub static SQL_CONNECT_ERRCODE: i8 = 3;

fn create_text_result_sql_connect_err(err_msg: Error)-> TextResult{
    let errmsg = err_msg.to_string();
    return TextResult {
        errmsg,
        errcode: CLAIMS_ERRCODE,
    };
}
// 在需要判断时候管理员时所需错误
pub static ADMINISTRATOR_ERRCODE: i8 = 4;
pub static ADMINISTRATOR_STR: &'static str = "您不是管理员，喵";

// 因为部分为常量，所以如此写
lazy_static! {

    pub static ref TEXT_SUCCESS_RESULT: TextResult = TextResult{
        errmsg: SUCCESS_STR.to_string(),
        errcode: SUCCESS_CODE
    };

    pub static ref TEXT_RESULT_ADMINISTRATOR_ERRCODE: TextResult = TextResult{
        errmsg: ADMINISTRATOR_STR.to_string(),
        errcode: ADMINISTRATOR_ERRCODE
    };

}



pub fn good_router() -> Router {
    return Router::new()
        .route("/addGood", post(add_good))
        .route("/getGoodList", post(get_good_list))
        .route("/delGood", post(del_good))
        .route("/updateGood", post(update_good))
        .route("/searchGood", post(search_good));
}

pub fn user_router() -> Router {
    return Router::new()
        .route("/login", post(login))
        .route("/signup", post(signup))
        .route("/autologin", post(autologin))
        .route("/delUser", post(del_user))
        .route("/searchUser", post(search_user))
        .route("/getUserList", post(get_user_list))
        .route("/updateUser", post(update_user));
}