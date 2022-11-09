use axum::{http::HeaderMap, Json};
use crate::claims::{claims_get_user};

use crate::sql::sqlite_util::sql_connection;

use crate::route::user::{create_user_list_result_claims_err, create_user_list_result_sql_connect_err, create_user_list_result_sql_err, create_user_list_success_result, USER_LIST_RESULT_ADMINISTRATOR_ERRCODE, UserListResult, UserName};
use crate::sql::user;


pub(in crate::route) async fn search_user(headers: HeaderMap, Json(user): Json<UserName>) -> Json<UserListResult> {
    return Json(match sql_connection().await {
        Ok(mut conn) => match claims_get_user(headers, &mut conn).await {
            Ok(login_user) => if login_user.is_administrator {
                match user::search_user(&mut conn, user.username).await {
                    Ok(user_list) => create_user_list_success_result(user_list),
                    Err(err) => create_user_list_result_sql_err(err)
                }
            }else{
                USER_LIST_RESULT_ADMINISTRATOR_ERRCODE.clone()
            }

            Err(errmsg) => create_user_list_result_claims_err(errmsg)
        }
        Err(err) => create_user_list_result_sql_connect_err(err)
    });
}
