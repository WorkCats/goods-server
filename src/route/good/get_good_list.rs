use axum::{http::HeaderMap, Json};
use crate::claims::claims_get_user;

use crate::sql::{
    sqlite_util::sql_connect,
    good::get_all_good,
};
use crate::route::good::{
    create_good_list_result_claims_err,
    create_good_list_result_sql_err,
    create_good_list_success_result,
    GOOD_LIST_RESULT_SQL_CONNECT_ERR,
    GoodListResult,
};

pub async fn get_good_list(headers: HeaderMap) -> Json<GoodListResult> {
    return Json(if let Some(mut conn) = sql_connect().await {
        match claims_get_user(headers, &mut conn).await {
            Ok(_) => match get_all_good(&mut conn).await {
                Ok(good_list) => create_good_list_success_result(good_list),
                Err(err) => create_good_list_result_sql_err(err)
            }
            Err(errmsg) => create_good_list_result_claims_err(errmsg)
        }
    } else {
        GOOD_LIST_RESULT_SQL_CONNECT_ERR.clone()
    });
}
