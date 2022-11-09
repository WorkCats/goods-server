use axum::http::HeaderMap;
use axum::Json;
use serde::{Deserialize, Serialize};
use crate::claims::claims_get_user;
use crate::route::{CLAIMS_ERRCODE, SQL_CONNECT_ERRCODE, SUCCESS_CODE, SUCCESS_STR};
use crate::sql::sqlite_util::sql_connection;

#[derive(Serialize, Deserialize)]
pub struct AdministratorResult {
    is_administrator: bool,
    errmsg: String,
    errcode: i8,
}

fn create_auto_login_success_result(
    is_administrator: bool,
) -> AdministratorResult {
    return AdministratorResult {
        is_administrator,
        errmsg: SUCCESS_STR.to_string(),
        errcode: SUCCESS_CODE,
    };
}

fn create_administrator_result_sql_connect_err(errmsg: String) -> AdministratorResult {
    return AdministratorResult {
        is_administrator: false,
        errmsg,
        errcode: SQL_CONNECT_ERRCODE,
    };
}

fn create_administrator_result_claims_err(errmsg: String) -> AdministratorResult {
    return AdministratorResult {
        is_administrator: false,
        errmsg,
        errcode: CLAIMS_ERRCODE,
    };
}

pub(in crate::route) async fn is_administrator(headers: HeaderMap) -> Json<AdministratorResult> {
    return Json(match sql_connection().await {
        Ok(mut conn) => match claims_get_user(headers, &mut conn).await {
            Ok(user) => create_auto_login_success_result(user.is_administrator),
            Err(err) => create_administrator_result_claims_err(err.to_string())
        }
        Err(err) => create_administrator_result_sql_connect_err(err.to_string()),
    });
}