use crate::sql::good::Good;

pub mod get_good_list;
pub mod add_good;
pub mod del_good;
pub mod update_good;
pub mod search_good;

pub const NULL_GOOD_LIST: Vec<Good> = Vec::new();
