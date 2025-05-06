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
    pub emergencyLatitude: BigDecimal,
}
impl Emergency {}
