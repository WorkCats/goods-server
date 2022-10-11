mod claims;
mod route;
mod sql;
mod data;

use data::DOMAIN;
use axum::{routing::post, Router, http};
use axum::http::{ HeaderMap, Method};
use crate::route::{good_router, user_router};
use tower_http::cors::CorsLayer;
use crate::data::ALLOW_ORIGIN;


#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", post(hello))
        .nest("/good", good_router())
        .nest("/user", user_router()).layer(
        // see https://docs.rs/tower-http/latest/tower_http/cors/index.html
        // for more details
        //
        // pay attention that for some request types like posting content-type: application/json
        // it is required to add ".allow_headers([http::header::CONTENT_TYPE])"
        // or see this issue https://github.com/tokio-rs/axum/issues/849
        CorsLayer::new()
            .allow_origin( ALLOW_ORIGIN.clone())
            .allow_credentials(true)
            .allow_headers([http::header::CONTENT_TYPE, http::header::AUTHORIZATION])
            .allow_methods( Method::POST),

    );

    println!("你可以通过打开 http://{}", DOMAIN);
    // run it with hyper on localhost:3000
    axum::Server::bind(&DOMAIN.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async  fn hello(headers: HeaderMap){
    println!("{:?}", headers);
}


