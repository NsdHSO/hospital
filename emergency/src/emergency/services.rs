use crate::emergency::Emergency;
use crate::error_handler::CustomError;
use crate::schema::emergency::dsl::emergency;
use crate::schema::emergency::emergencyIc;
use diesel::prelude::*;
use crate::db::config;

pub struct EmergencyService;

impl EmergencyService {
    pub fn find_one(emergency_ic: &str) -> Result<Option<Emergency>, CustomError> {
        let mut conn = config::connection()?;

        let record = emergency
            .filter(emergencyIc.eq(emergency_ic))
            .select(Emergency::as_select())
            .first(&mut conn)
            .optional()?;

        println!("Query result: {:?}", record);

        Ok(record)
    }
}
