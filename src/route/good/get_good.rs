use axum::{http::HeaderMap, Json};
use crate::claims::claims_get_user;
use serde::{Deserialize, Serialize};
use crate::sql::{sqlite_util::sql_connection };
use sqlx::Error;
use crate::route::{SUCCESS_STR, SUCCESS_CODE, SQL_ERRCODE, CLAIMS_ERRCODE, SQL_CONNECT_ERRCODE};
use crate::route::good::GoodId;
use crate::sql::good::{get_good_by_id, Good};

/// 在搜索和获取全部货物时使用
/// `good` 货物
/// `errmsg` 错误日志
/// `errcode` 错误类型，用与判断那些内容是错误的
#[derive(Serialize, Deserialize)]
pub struct GoodResult {
    good: Option<Good>,
    errmsg: String,
    errcode: i8,
}

/// 成功请求时使用
pub(crate) fn create_good_success_result(good: Option<Good>) -> GoodResult {
    return GoodResult {
        good,
        errmsg: SUCCESS_STR.to_string(),
        errcode: SUCCESS_CODE,
    };
}

/// 出现 Sql 问题时使用
pub(crate) fn create_good_result_sql_err(err_msg: Error) -> GoodResult {
    let errmsg = err_msg.to_string();
    return GoodResult {
        good: None,
        errmsg,
        errcode: SQL_ERRCODE,
    };
}

/// Claims 出现问题时使用
pub(crate) fn create_good_result_claims_err(errmsg: String) -> GoodResult {
    return GoodResult {
        good: None,
        errmsg,
        errcode: CLAIMS_ERRCODE,
    };
}

/// sql connect 出现问题时使用
pub(crate) fn create_good_result_sql_connect_err(errmsg: String) -> GoodResult {
    return GoodResult {
        good: None,
        errmsg,
        errcode: SQL_CONNECT_ERRCODE
    };
}

pub(in crate::route) async fn get_good(headers: HeaderMap, Json(good): Json<GoodId>) -> Json<GoodResult> {
    return Json(match sql_connection().await {
        Ok(mut conn) => match claims_get_user(headers, &mut conn).await {
            Ok(_) => match get_good_by_id(&mut conn, good.good_id).await {
                Ok(good) => create_good_success_result(Some(good)),
                Err(err) => create_good_result_sql_err(err)
            }

            Err(errmsg) => create_good_result_claims_err(errmsg)
        }

        Err(err) => create_good_result_sql_connect_err(err)
    });
}