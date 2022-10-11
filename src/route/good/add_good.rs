use axum::{Json, http::HeaderMap};
use crate::claims::claims_get_user;

use crate::sql::{
    good::{insert_good, Good},
    sqlite_util::sql_connect,
};
use crate::route::{
    create_text_result_claims_err,
    create_text_result_sql_err,
    TEXT_RESULT_SQL_CONNECT_ERR,
    TEXT_SUCCESS_RESULT,
    TextResult,
};

pub async fn add_good(headers: HeaderMap, Json(good): Json<Good>) -> Json<TextResult> {
    return Json(if let Some(mut conn) = sql_connect().await {
        match claims_get_user(headers, &mut conn).await {
            Ok(_) => match insert_good(&mut conn, good).await {
                Ok(_) => TEXT_SUCCESS_RESULT.clone(),
                Err(err) => create_text_result_sql_err(err)
            }
            Err(errmsg) => create_text_result_claims_err(errmsg)
        }
    } else {
        TEXT_RESULT_SQL_CONNECT_ERR.clone()
    });
}

