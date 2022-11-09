use axum::{http::HeaderMap, Json};
use crate::claims::claims_get_user;

use crate::sql::{
    sqlite_util::sql_connection
};

use crate::route::{create_text_result_claims_err, create_text_result_sql_connect_err, create_text_result_sql_err, create_text_result_success, TextResult};
use crate::route::good::GoodId;
use crate::sql;


pub(in crate::route) async fn has_good(headers: HeaderMap, Json(good): Json<GoodId>) -> Json<TextResult> {
    return Json(match sql_connection().await {
        Ok(mut conn) => match claims_get_user(headers, &mut conn).await {
            Ok(_) => match sql::good::select_good_has(&mut conn, good.good_id).await {
                Ok(has_good) => create_text_result_success(has_good.to_string()),
                Err(err) => create_text_result_sql_err(err)
            }
            Err(errmsg) => create_text_result_claims_err(errmsg)
        }

        Err(err) => create_text_result_sql_connect_err(err)
    });
}