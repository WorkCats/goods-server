mod user;
mod good;

use axum::{routing::post, Router};

use good::add_good::add_good;
use good::get_good_list::get_good_list;
use good::del_good::del_good;
use good::update_good::update_good;
use crate::route::good::search_good::search_good;
use crate::route::user::login::login;
use crate::route::user::signup::signup;
use crate::route::user::autologin::autologin;



pub fn good_router() -> Router {
    return Router::new()
        .route("/addGood", post(add_good))
        .route("/getGoodList", post(get_good_list))
        .route("/delGood", post(del_good))
        .route("/updateGood", post(update_good))
        .route("/searchGood", post(search_good));
}

pub fn user_router() -> Router {
    return Router::new()
        .route("/login", post(login))
        .route("/signup", post(signup))
        .route("/autologin", post(autologin));
}