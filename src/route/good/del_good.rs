use axum::{http::HeaderMap, Json};
use crate::claims::claims_get_user;

use crate::sql::{
    good::delete_good,
    sqlite_util::sql_connect,
};
use crate::route::{
    good::GoodName,
    create_text_result_claims_err,
    create_text_result_sql_err,
    TEXT_RESULT_SQL_CONNECT_ERR,
    TEXT_SUCCESS_RESULT,
    TextResult,
};


pub async fn del_good(headers: HeaderMap, Json(good): Json<GoodName>) -> Json<TextResult> {
    return Json(if let Some(mut conn) = sql_connect().await {
        match claims_get_user(headers, &mut conn).await {
            Ok(_) => match delete_good(&mut conn, good.good_name).await {
                Ok(_) => TEXT_SUCCESS_RESULT.clone(),
                Err(err) => create_text_result_sql_err(err)
            }

            Err(errmsg) =>
                create_text_result_claims_err(errmsg)
        }
    } else {
        TEXT_RESULT_SQL_CONNECT_ERR.clone()
    });
}

