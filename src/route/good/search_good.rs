use axum::{http::HeaderMap, Json};
use crate::claims::{claims_get_user};

use crate::sql::{
    good::select_good,
    sqlite_util::sql_connect,
};
use crate::route::good::{create_good_list_result_claims_err, create_good_list_result_sql_err, create_good_list_success_result, create_good_list_result_sql_connect_err, GoodListResult, GoodName};


pub async fn search_good(headers: HeaderMap, Json(good): Json<GoodName>) -> Json<GoodListResult> {
    return Json(match sql_connect().await {
        Ok(mut conn) => match claims_get_user(headers, &mut conn).await {
            Ok(_) => match select_good(&mut conn, good.good_name).await {
                Ok(good_list) => create_good_list_success_result(good_list),
                Err(err) => create_good_list_result_sql_err(err)
            }

            Err(errmsg) => create_good_list_result_claims_err(errmsg)
        }
        Err(err) => create_good_list_result_sql_connect_err(err)
    });
}

