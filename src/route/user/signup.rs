use axum::{http::HeaderMap, Json};
use crate::claims::claims_get_user;

use crate::sql::{
    user::{insert_user, User},
    sqlite_util::sql_connection,
};
use crate::route::{create_text_result_claims_err, create_text_result_sql_connect_err, create_text_result_sql_err, TEXT_RESULT_ADMINISTRATOR_ERRCODE, TEXT_SUCCESS_RESULT, TextResult};


pub(in crate::route) async fn signup(headers: HeaderMap, Json(signup_user): Json<User>) -> Json<TextResult> {
    return Json(match sql_connection().await {
        Ok(mut conn) => match claims_get_user(headers, &mut conn).await {
            Ok(login_user) =>
                if login_user.is_administrator {
                    match insert_user(&mut conn, signup_user).await {
                        Ok(_) => TEXT_SUCCESS_RESULT.clone(),

                        Err(err) => create_text_result_sql_err(err)
                    }
                } else {
                    TEXT_RESULT_ADMINISTRATOR_ERRCODE.clone()
                }

            Err(errmsg) => create_text_result_claims_err(errmsg)
        }
        Err(err) => create_text_result_sql_connect_err(err)
    }
    );
}