use crate::db::config;
use crate::emergency::Emergency;
use crate::error_handler::CustomError;
use crate::schema::emergency::dsl::emergency;
use crate::schema::emergency::emergencyIc;
use diesel::prelude::*;

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

    pub fn find_all(page: i64, per_page: i64) -> Result<Vec<Emergency>, CustomError> {
        let mut conn = config::connection()?;

        let offset = (page - 1) * per_page;

        let records = emergency
            .limit(per_page)
            .offset(offset)
            .select(Emergency::as_select())
            .load(&mut conn)?;

        Ok(records)
    }
}
