use crate::emergency::enums::enum_emergency::EmergencyStatus;
use crate::schema::emergency;
use bigdecimal::BigDecimal;
use nanoid::nanoid;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct NewEmergencyRequest {
    pub description: String,
    pub reportedBy: Option<i32>,
    pub notes: Option<String>,
    pub idAmbulance: Option<uuid::Uuid>,
    pub additional_info: Option<String>,
    pub emergencyLongitude: BigDecimal,
    pub emergencyLatitude: BigDecimal,
}


impl From<NewEmergencyRequest> for Emergency {
    fn from(req: NewEmergencyRequest) -> Self {
        let mut emergency = Emergency {
            emergencyIc: String::new(), // Will be set by generate_id
            description: req.description,
            reportedBy: req.reportedBy,
            notes: req.notes,
            status: EmergencyStatus::Pending,
            idAmbulance: req.idAmbulance,
            additional_info: req.additional_info,
            emergencyLongitude: req.emergencyLongitude,
            emergencyLatitude: req.emergencyLatitude,
        };
        emergency.generate_id();
        emergency
    }
}

#[derive(Debug, Serialize, Deserialize, Insertable, Queryable, Selectable, QueryId)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[table_name = "emergency"]
pub struct Emergency {
    pub emergencyIc: String,
    pub description: String,
    pub reportedBy: Option<i32>,
    pub notes: Option<String>,
    pub status: EmergencyStatus,  // Add this
    pub idAmbulance: Option<uuid::Uuid>,
    pub additional_info: Option<String>,
    pub emergencyLongitude: BigDecimal,
    pub emergencyLatitude: BigDecimal,
}

impl Emergency {
    pub fn generate_id(&mut self) {
        self.emergencyIc = nanoid!(10, &['1', '2', '3', '4', '5', '6', '7', '8', '9', '0']);
    }
}
