
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use bigdecimal::BigDecimal;
use diesel::deserialize::FromSql;
use diesel::serialize::ToSql;
use diesel::sql_types::Jsonb;
use diesel::pg::Pg;

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
#[diesel(table_name = crate::schema::ambulance)]
pub struct Ambulance {
    pub id: Uuid,
    pub hospitalId: String,
    pub ambulanceIc: i32,
    pub vehicleNumber: String,
    pub make: Option<String>,
    pub year: Option<i32>,
    pub capacity: Option<i32>,
    #[diesel(sql_type = crate::schema::sql_types::AmbulanceTypeEnum)]
    pub type_: AmbulanceType,
    #[diesel(sql_type = crate::schema::sql_types::AmbulanceStatusEnum)]
    pub status: AmbulanceStatus,
    pub mission: Option<String>,
    #[diesel(serialize_as = Option<Json<serde_json::Value>>)]
    pub passengers: Option<serde_json::Value>,
    pub driverName: Option<String>,
    pub driverLicense: Option<String>,
    pub mileage: Option<i32>,
    pub fuelType: Option<String>,
    pub registrationNumber: Option<String>,
    pub insuranceProvider: Option<String>,
    pub notes: Option<String>,
    #[diesel(sql_type = crate::schema::sql_types::AmbulanceCardetailsmakeEnum)]
    pub carDetailsMake: AmbulanceCarDetailsMake,
    #[diesel(sql_type = crate::schema::sql_types::AmbulanceCardetailsmodelEnum)]
    pub carDetailsModel: AmbulanceCarDetailsModel,
    pub carDetailsYear: i32,
    pub carDetailsColor: String,
    pub carDetailsIsambulance: bool,
    pub carDetailsLicenseplate: Option<String>,
    pub carDetailsMileage: Option<f64>,
    pub locationLatitude: BigDecimal,
    pub locationLongitude: BigDecimal,
}

#[derive(Debug, Serialize, Deserialize, FromSqlRow, AsExpression)]
#[diesel(sql_type = Jsonb)]
struct Json<T>(T);

#[derive(Debug, Serialize, Deserialize, diesel_derive_enum::DbEnum)]
#[ExistingTypePath = "crate::schema::sql_types::AmbulanceTypeEnum"]
pub enum AmbulanceType {
    Basic,
    Advanced,
    Critical,
}

#[derive(Debug, Serialize, Deserialize, diesel_derive_enum::DbEnum)]
#[ExistingTypePath = "crate::schema::sql_types::AmbulanceStatusEnum"]
pub enum AmbulanceStatus {
    Available,
    OnMission,
    Maintenance,
    OutOfService,
}

#[derive(Debug, Serialize, Deserialize, diesel_derive_enum::DbEnum)]
#[ExistingTypePath = "crate::schema::sql_types::AmbulanceCardetailsmakeEnum"]
pub enum AmbulanceCarDetailsMake {
    Toyota,
    Ford,
    MercedesBenz,
    Volkswagen,
}

#[derive(Debug, Serialize, Deserialize, diesel_derive_enum::DbEnum)]
#[ExistingTypePath = "crate::schema::sql_types::AmbulanceCardetailsmodelEnum"]
pub enum AmbulanceCarDetailsModel {
    Sprinter,
    Transit,
    HiAce,
    Crafter,
}

impl FromSql<Jsonb, Pg> for Json<serde_json::Value> {
    fn from_sql(bytes: diesel::backend::RawValue<'_, Pg>) -> diesel::deserialize::Result<Self> {
        let value = <serde_json::Value as FromSql<Jsonb, Pg>>::from_sql(bytes)?;
        Ok(Json(value))
    }
}

impl ToSql<Jsonb, Pg> for Json<serde_json::Value> {
    fn to_sql<'b>(&'b self, out: &mut diesel::serialize::Output<'b, '_, Pg>) -> diesel::serialize::Result {
        ToSql::<Jsonb, Pg>::to_sql(&self.0, out)
    }
}