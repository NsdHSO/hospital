use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct PaginationParams {
    #[serde(default = "page")]
    pub page: i64,
    #[serde(default = "per_page")]
    pub per_page: i64,
    #[serde(default)]
    pub filter: Option<String>,

}

fn page() -> i64 {
    1
}
fn per_page() -> i64 {
    10
}

#[derive(Serialize)]
pub struct PaginatedResponse<T> {
    pub data: T,
    pub pagination: PaginationInfo,
}

#[derive(Serialize)]
pub struct PaginationInfo {
    pub current_page: i64,
    pub page_size: i64,
    pub total_items: i64,
    pub total_pages: i64,
    pub has_next_page: bool,
    pub has_previous_page: bool,
}
