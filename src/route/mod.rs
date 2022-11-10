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
    has_good::has_good
};
use crate::route::good::get_good::get_good;
use crate::route::user::{
    login::login,
    signup::signup,
    autologin::autologin,
    del_user::del_user
};
use crate::route::user::get_goods::get_goods;
use crate::route::user::get_user_list::get_user_list;
use crate::route::user::get_username_list::get_username_list;
use crate::route::user::is_administrator::is_administrator;
use crate::route::user::search_user::search_user;
use crate::route::user::update_user::update_user;

/// TextResult 是在客户端与服务端交互，不需要返回数据时使用的结构体
/// 告诉客户端本次结果，如果未发生错误 errcode = 0, errmsg = ""
/// 反之则会有对应的错误内容

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

// 成功时的 CODE
static SUCCESS_CODE: i8 = 0;
static SUCCESS_STR: &'static str = "";

// 在 SQL 语句出错时所需错误
static SQL_ERRCODE: i8 = 1;
fn create_text_result_success(errmsg: String) -> TextResult {
    return TextResult {
        errmsg,
        errcode: SUCCESS_CODE,
    };
}

/// 在 SQL 语句发生错误时调用
/// `err_msg` 错误信息
fn create_text_result_sql_err(err_msg: Error) -> TextResult {
    let errmsg = err_msg.to_string();
    return TextResult {
        errmsg,
        errcode: SQL_ERRCODE,
    };
}

// 在 CLAIMS 检测时所需错误
static CLAIMS_ERRCODE: i8 = 2;

/// 在 CLAIMS 检测时发生错误时被调用
/// `err_msg` 错误信息
fn create_text_result_claims_err(errmsg: String) -> TextResult {
    return TextResult {
        errmsg,
        errcode: CLAIMS_ERRCODE,
    };
}

// 在 SQL 连接出现问题时所需错误
static SQL_CONNECT_ERRCODE: i8 = 3;

/// 在 SQL 连接出现问题时被调用
/// `err_msg` 错误信息
fn create_text_result_sql_connect_err(errmsg: String) -> TextResult {
    return TextResult {
        errmsg,
        errcode: CLAIMS_ERRCODE,
    };
}

// 在需要判断时候管理员时所需错误
static ADMINISTRATOR_ERRCODE: i8 = 4;
static ADMINISTRATOR_STR: &'static str = "您不是管理员，喵";

// 因为部分为常量，所以如此写
lazy_static! {
    /// 在删除，更新等模块执行成功时返回的内容
     static ref TEXT_SUCCESS_RESULT: TextResult = TextResult{
        errmsg: SUCCESS_STR.to_string(),
        errcode: SUCCESS_CODE
    };

    /// 非管理员权限
    static ref TEXT_RESULT_ADMINISTRATOR_ERRCODE: TextResult = TextResult{
        errmsg: ADMINISTRATOR_STR.to_string(),
        errcode: ADMINISTRATOR_ERRCODE
    };
}

/// good 对应的路由
pub(super) fn good_router() -> Router {
    return Router::new()
        .route("/addGood", post(add_good))
        .route("/getGoodList", post(get_good_list))
        .route("/delGood", post(del_good))
        .route("/updateGood", post(update_good))
        .route("/searchGood", post(search_good))
        .route("/hasGood", post(has_good))
        .route("/getGoodById", post(get_good));
}

/// user 对应的路由
pub(super) fn user_router() -> Router {
    return Router::new()
        .route("/login", post(login))
        .route("/signup", post(signup))
        .route("/autologin", post(autologin))
        .route("/delUser", post(del_user))
        .route("/searchUser", post(search_user))
        .route("/getUserList", post(get_user_list))
        .route("/updateUser", post(update_user))
        .route("/getUsernameList", post(get_username_list))
        .route("/getGoods", post(get_goods))
        .route("/isAdministrator", post(is_administrator));
}