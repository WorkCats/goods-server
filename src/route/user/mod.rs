use serde::{Deserialize, Serialize};

pub mod signup;
pub mod login;
pub mod autologin;
pub mod del_user;

#[derive(Serialize, Deserialize)]
pub struct UserName {
    username: String,
}
