use axum::{http::HeaderMap, Json};
use crate::claims::claims_get_user;
use crate::route::user::{create_user_list_result_claims_err, create_user_list_result_sql_connect_err, create_user_list_result_sql_err, create_user_list_success_result, USER_LIST_RESULT_ADMINISTRATOR_ERRCODE, UserListResult};
use crate::sql::sqlite_util::sql_connect;
use crate::sql::user::get_all_user;


pub async fn get_user_list(headers: HeaderMap) -> Json<UserListResult> {
    return Json(match sql_connect().await {
        Ok(mut conn) => match claims_get_user(headers, &mut conn).await {
            Ok(login_user) => if login_user.is_administrator {
                match get_all_user(&mut conn).await {
                    Ok(good_list) => create_user_list_success_result(good_list),
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
