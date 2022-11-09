use axum::{http::HeaderMap, Json};

use crate::claims::claims_get_user;
use crate::sql::{sqlite_util::sql_connect, user, user::User};
use crate::route::{create_text_result_claims_err, create_text_result_sql_connect_err, create_text_result_sql_err, TEXT_RESULT_ADMINISTRATOR_ERRCODE, TEXT_SUCCESS_RESULT, TextResult};


pub(in crate::route) async fn update_user(headers: HeaderMap, Json(user): Json<User>) -> Json<TextResult> {
    return Json(match sql_connect().await {
        Ok(mut conn) => match claims_get_user(headers, &mut conn).await {
            Ok(login_user) => if login_user.is_administrator {
                match user::update_user(&mut conn, user).await {
                    Ok(_) => TEXT_SUCCESS_RESULT.clone(),

                    Err(err) => create_text_result_sql_err(err)
                }
            } else {
                TEXT_RESULT_ADMINISTRATOR_ERRCODE.clone()
            }

            Err(errmsg) => create_text_result_claims_err(errmsg)
        }
        Err(err) => create_text_result_sql_connect_err(err)
    });
}
