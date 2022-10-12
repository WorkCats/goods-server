pub mod get_good_list;
pub mod add_good;
pub mod del_good;
pub mod update_good;
pub mod search_good;

use serde::{Deserialize, Serialize};
use sqlx::Error;
use crate::route::{
    SQL_ERRCODE,
    SUCCESS_CODE,
    SUCCESS_STR,
    SQL_CONNECT_ERRCODE,
    CLAIMS_ERRCODE
};

use crate::sql::good::Good;

pub const NULL_GOOD_LIST: Vec<Good> = Vec::new();

#[derive(Serialize, Deserialize)]
pub struct GoodName {
    good_name: String,
}

#[derive(Serialize, Deserialize)]
pub struct GoodListResult {
    good_list: Vec<Good>,
    errmsg: String,
    errcode: i8,
}
impl Clone for GoodListResult{
    fn clone(&self) -> GoodListResult {
        GoodListResult {
            good_list: (*self.good_list).to_vec(),
            errmsg: (self.errmsg).parse().unwrap(),
            errcode: self.errcode
        }
    }
}


pub(crate) fn create_good_list_success_result(good_list: Vec<Good>) -> GoodListResult {
    return GoodListResult {
        good_list,
        errmsg: SUCCESS_STR.to_string(),
        errcode: SUCCESS_CODE,
    };
}

pub(crate) fn create_good_list_result_sql_err(err_msg: Error) -> GoodListResult {
    let errmsg = err_msg.to_string();
    return GoodListResult {
        good_list: NULL_GOOD_LIST,
        errmsg,
        errcode: SQL_ERRCODE,
    };
}

pub(crate) fn create_good_list_result_claims_err(errmsg: String) -> GoodListResult {
    return GoodListResult {
        good_list: NULL_GOOD_LIST,
        errmsg,
        errcode: CLAIMS_ERRCODE,
    };
}

pub(crate) fn create_good_list_result_sql_connect_err(errmsg: String) -> GoodListResult {
    return GoodListResult {
        good_list: NULL_GOOD_LIST,
        errmsg,
        errcode: SQL_CONNECT_ERRCODE
    };
}
