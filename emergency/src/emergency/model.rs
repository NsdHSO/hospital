use crate::schema::emergency;
use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Insertable, Queryable, Selectable)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[table_name = "emergency"]
pub struct Emergency {
    pub emergencyIc: String,
    pub description: String,
    pub reportedBy: Option<i32>,
    pub notes: Option<String>,
    pub idAmbulance: Option<uuid::Uuid>,
    pub additional_info: Option<String>,
    pub emergencyLongitude: BigDecimal,
    pub emergencyLatitude: BigDecimal,
}
impl Emergency {}


#[derive(Deserialize)]
pub struct PaginationParams {
    #[serde(default = "page")]
    pub page: i64,
    #[serde(default = "per_page")]
    pub per_page: i64,
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
