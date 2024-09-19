use serde::Deserialize;

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

impl Default for OptionPagination {
    fn default() -> Self {
        OptionPagination {
            page: Some(0),
            size: Some(10),
        }
    }
}

impl OptionPagination {
    pub fn default(&self) -> Pagination {
        Pagination {
            page: self.page.unwrap_or(0),
            size: self.size.unwrap_or(10),
        }
    }
}
