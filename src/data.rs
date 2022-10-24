use axum::http::{HeaderValue};

// 对应域名
pub static DOMAIN: &'static str = "0.0.0.0:3000";

// 加密密钥
pub static DECODING_KEY: &'static str = "agoines";

// SQL 对应的路径
pub static SQL_FILE: &'static str = "./data/goods.sqlite";

// 允许跨域的域名
pub static ORIGIN:[&'static str; 2] = [
    "http://localhost:4000",
    "http://10.147.17.233:4000/"
];
pub static ALLOW_ORIGIN: [HeaderValue; 2] = [
    HeaderValue::from_static(ORIGIN[0]),
    HeaderValue::from_static(ORIGIN[1])
];

pub static DEFAULT_USERNAME: &'static str = "agoines";
pub static DEFAULT_PASSWORD: &'static str = "qwer1234";