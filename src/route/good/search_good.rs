use axum::{http::HeaderMap, Json};
use serde::{Deserialize, Serialize};
use crate::claims::{claims_get_user};

use crate::sql::{
    good::select_good,
    sqlite_util::sql_connection,
};
use crate::route::good::{create_good_list_result_claims_err, create_good_list_result_sql_err, create_good_list_success_result, create_good_list_result_sql_connect_err, GoodListResult};

/// 在搜索和获取全部货物时使用
/// `good_name` 货物名称，在搜索中使用
#[derive(Serialize, Deserialize)]
pub struct GoodName {
    good_name: String,
}

/// `headers` 请求头，获取请求头的 token 用于校验
/// `Json(good)` 对请求中的 json 进行反序列化
pub(in crate::route) async fn search_good(headers: HeaderMap, Json(good): Json<GoodName>) -> Json<GoodListResult> {
    return Json(match sql_connection().await {
        Ok(mut conn) => match claims_get_user(headers, &mut conn).await {
            // 在数据库中进行搜索
            Ok(_) => match select_good(&mut conn, good.good_name).await {
                Ok(good_list) => create_good_list_success_result(good_list),
                Err(err) => create_good_list_result_sql_err(err)
            }
            Err(errmsg) => create_good_list_result_claims_err(errmsg)
        }
        Err(err) => create_good_list_result_sql_connect_err(err)
    });
}

