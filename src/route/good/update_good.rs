use axum::{http::HeaderMap, Json};

use crate::claims::claims_get_user;
use crate::sql::good;
use crate::sql::{
    sqlite_util::sql_connect,
    good::Good,
};
use crate::route::{create_text_result_claims_err, create_text_result_sql_connect_err, create_text_result_sql_err, TEXT_SUCCESS_RESULT, TextResult};

pub(in crate::route) async fn update_good(headers: HeaderMap, Json(good): Json<Good>) -> Json<TextResult> {
    return Json(match sql_connect().await {
        Ok(mut conn) => match claims_get_user(headers, &mut conn).await {
            Ok(_) => match good::update_good(&mut conn, good).await {
                Ok(_) => TEXT_SUCCESS_RESULT.clone(),
                Err(err) => create_text_result_sql_err(err)
            }
            Err(errmsg) => create_text_result_claims_err(errmsg)
        }
        Err(err) => create_text_result_sql_connect_err(err)
    });
}
