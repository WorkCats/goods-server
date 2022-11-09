use axum::{http::HeaderMap, Json};
use serde::{Deserialize, Serialize};
use crate::claims::claims_get_user;

use crate::sql::{
    good::delete_good,
    sqlite_util::sql_connection,
};

use crate::route::{create_text_result_claims_err, create_text_result_sql_err,  TEXT_SUCCESS_RESULT, TextResult, create_text_result_sql_connect_err};

#[derive(Serialize, Deserialize)]
pub struct GoodId {
    good_id: String,
}
pub(in crate::route) async fn del_good(headers: HeaderMap, Json(good): Json<GoodId>) -> Json<TextResult> {
    return Json(match sql_connection().await {
        Ok(mut conn) => match claims_get_user(headers, &mut conn).await {
            Ok(_) => match delete_good(&mut conn, good.good_id).await {
                Ok(_) => TEXT_SUCCESS_RESULT.clone(),
                Err(err) => create_text_result_sql_err(err)
            }

            Err(errmsg) => create_text_result_claims_err(errmsg)
        }

        Err(err) => create_text_result_sql_connect_err(err)
    });
}

