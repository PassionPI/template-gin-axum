use serde::Deserialize;

pub mod todo;
pub mod user;

#[derive(Deserialize, Debug)]
pub struct Pagination {
    pub page: i32,
    pub size: i32,
}

#[derive(Deserialize, Debug)]
pub struct OptionPagination {
    pub page: Option<i32>,
    pub size: Option<i32>,
}

pub fn rm_pagination_option(option_pagination: OptionPagination) -> Pagination {
    Pagination {
        page: option_pagination.page.unwrap_or(0),
        size: option_pagination.size.unwrap_or(10),
    }
}
