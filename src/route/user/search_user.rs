use axum::{http::HeaderMap, Json};
use crate::claims::{claims_get_user};

use crate::sql::sqlite_util::sql_connect;

use crate::route::user::{create_user_list_result_claims_err, create_user_list_result_sql_connect_err, create_user_list_result_sql_err, create_user_list_success_result, UserListResult, UserName};
use crate::sql::user::select_user;


pub async fn search_user(headers: HeaderMap, Json(user): Json<UserName>) -> Json<UserListResult> {
    return Json(match sql_connect().await {
        Ok(mut conn) => match claims_get_user(headers, &mut conn).await {
            Ok(_) => match select_user(&mut conn, user.username).await {
                Ok(user_list) => create_user_list_success_result(user_list),
                Err(err) => create_user_list_result_sql_err(err)
            }

            Err(errmsg) => create_user_list_result_claims_err(errmsg)
        }
        Err(err) => create_user_list_result_sql_connect_err(err)
    });
}
