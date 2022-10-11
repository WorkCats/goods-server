use axum::{http::HeaderMap, Json};

use crate::claims::claims_get_user;
use crate::sql::{
    user::delete_user,
    sqlite_util::sql_connect,
};
use crate::route::{
    user::UserName,
    create_text_result_claims_err,
    create_text_result_sql_err,
    TEXT_RESULT_ADMINISTRATOR_ERRCODE,
    TEXT_RESULT_SQL_CONNECT_ERR,
    TEXT_SUCCESS_RESULT,
    TextResult,
};


pub async fn del_user(headers: HeaderMap, Json(del_user): Json<UserName>) -> Json<TextResult> {
    return Json(
        if let Some(mut conn) = sql_connect().await {
            match claims_get_user(headers, &mut conn).await {
                Ok(login_user) =>
                    if login_user.is_administrator {
                        match delete_user(&mut conn, del_user.username).await {
                            Ok(_) => TEXT_SUCCESS_RESULT.clone(),
                            Err(err) => create_text_result_sql_err(err)
                        }
                    } else {
                        TEXT_RESULT_ADMINISTRATOR_ERRCODE.clone()
                    }

                Err(errmsg) => create_text_result_claims_err(errmsg)
            }
        } else {
            TEXT_RESULT_SQL_CONNECT_ERR.clone()
        });
}
