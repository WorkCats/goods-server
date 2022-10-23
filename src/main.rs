mod claims;
mod route;
mod sql;
mod data;

use axum::{Router, http::{Method, header::{CONTENT_TYPE, AUTHORIZATION}}, Server};
use axum::response::{Html, IntoResponse};
use axum::routing::{get};

use askama::Template;
use axum::http::{header, HeaderMap};
use std::include_str;
use tower_http::cors::CorsLayer;

use crate::data::{DOMAIN, ALLOW_ORIGIN};
use crate::header::HeaderValue;
use crate::route::{good_router, user_router};


#[tokio::main]
async fn main() {

    let app = Router::new()
        .nest("/good", good_router())
        .nest("/user", user_router())
        .nest("/web", get(index))
        .route("/static/index.css", get(index_app_css))
        .route("/static/index.js", get(index_app_js))
        .layer(
        // see https://docs.rs/tower-http/latest/tower_http/cors/index.html
        // for more details
        //
        // pay attention that for some request types like posting content-type: application/json
        // it is required to add ".allow_headers([http::header::CONTENT_TYPE])"
        // or see this issue https://github.com/tokio-rs/axum/issues/849
        CorsLayer::new()
            .allow_origin(ALLOW_ORIGIN.clone())
            .allow_credentials(true)
            .allow_headers([CONTENT_TYPE, AUTHORIZATION])
            .allow_methods([Method::POST, Method::GET]),
    );

    println!("你可以通过打开 http://{}", DOMAIN);
    // run it with hyper on localhost:3000
    Server::bind(&DOMAIN.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {}

async fn index() -> Html<String> {
    let tpl = IndexTemplate {};
    return match tpl.render().map_err(|err| err.to_string()) {
        Ok(html) => {
            Html(html)
        }
        Err(err) =>{
            Html(err)
        }
    }
}

const CSS_INDEX_APP: &str = include_str!("../static/index.css");
const JS_INDEX_APP: &str = include_str!("../static/index.js");

async fn asset(source: &'static [u8], ty: &'static str) -> impl IntoResponse {
    let mut headermap = HeaderMap::new();
    headermap.insert(CONTENT_TYPE, HeaderValue::from_static(ty));
    (headermap, source)
}

async fn js(source: &'static str) -> impl IntoResponse {
    asset(source.as_bytes(), mime::TEXT_JAVASCRIPT.as_ref()).await
}

async fn css(source: &'static str) -> impl IntoResponse {
    asset(source.as_bytes(), mime::TEXT_CSS.as_ref()).await
}

pub async fn index_app_js() -> impl IntoResponse {
    js(JS_INDEX_APP).await
}

pub async fn index_app_css() -> impl IntoResponse {
    css(CSS_INDEX_APP).await
}