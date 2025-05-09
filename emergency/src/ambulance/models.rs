use crate::ambulance::enums::{
    AmbulanceCarDetailsMake, AmbulanceCarDetailsModel, AmbulanceStatus, AmbulanceType,
};
use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use diesel::deserialize::FromSql;
use diesel::pg::Pg;
use diesel::prelude::*;
use diesel::prelude::*;
use diesel::serialize::ToSql;
use diesel::sql_types::{Integer, Jsonb, Text};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromSqlRow, AsExpression)]
#[diesel(sql_type = Jsonb)]
pub struct Json<T>(pub T);

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable, Selectable)]
#[diesel(table_name = crate::schema::ambulance)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Ambulance {
    pub createdAt: NaiveDateTime,
    pub updatedAt: NaiveDateTime,
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
    pub passengers: Option<Json<serde_json::Value>>,
    pub driverName: Option<String>,
    pub driverLicense: Option<String>,
    pub lastServiceDate: Option<NaiveDateTime>,
    pub nextServiceDate: Option<NaiveDateTime>,
    pub mileage: Option<i32>,
    pub fuelType: Option<String>,
    pub registrationNumber: Option<String>,
    pub insuranceProvider: Option<String>,
    pub insuranceExpiryDate: Option<NaiveDateTime>,
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

impl FromSql<Jsonb, Pg> for Json<serde_json::Value> {
    fn from_sql(bytes: diesel::backend::RawValue<'_, Pg>) -> diesel::deserialize::Result<Self> {
        let value = <serde_json::Value as FromSql<Jsonb, Pg>>::from_sql(bytes)?;
        Ok(Json(value))
    }
}

impl ToSql<Jsonb, Pg> for Json<serde_json::Value> {
    fn to_sql<'b>(
        &'b self,
        out: &mut diesel::serialize::Output<'b, '_, Pg>,
    ) -> diesel::serialize::Result {
        ToSql::<Jsonb, Pg>::to_sql(&self.0, out)
    }
}
