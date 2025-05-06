use diesel::dsl::sql;
use crate::db::config;
use crate::emergency::{Emergency, PaginatedResponse, PaginationInfo};
use crate::error_handler::CustomError;
use crate::schema::emergency::dsl::emergency;
use crate::schema::emergency::emergencyIc;
use diesel::prelude::*;
use diesel::sql_types::BigInt;
use crate::db::config::DbConnection;
pub struct EmergencyService {
    conn: DbConnection,
}

impl EmergencyService {
    pub fn new() -> Result<Self, CustomError> {
        let conn = config::connection()?;
        Ok(EmergencyService { conn })
    }

    pub fn find_one(&mut self, emergency_ic: &str) -> Result<Option<Emergency>, CustomError> {
        let record = emergency
            .filter(emergencyIc.eq(emergency_ic))
            .select(Emergency::as_select())
            .first(&mut self.conn)
            .optional()?;

        println!("Query result: {:?}", record);

        Ok(record)
    }

    pub fn find_all(&mut self, page: i64, per_page: i64) -> Result<PaginatedResponse<Vec<Emergency>>, CustomError> {

        let offset = (page - 1) * per_page;

        // Single query that gets both count and records
        let records_with_count: Vec<(Emergency, i64)> = emergency
            .limit(per_page)
            .offset(offset)
            .select((Emergency::as_select(), sql::<BigInt>("COUNT(*) OVER()")))
            .load(&mut self.conn)?;

        let total = records_with_count.first().map(|(_,count)| count).unwrap_or(&0).to_owned();
        let records = records_with_count.into_iter().map(|(record, _)| record).collect();
        let total_pages = (total as f64 / per_page as f64).ceil() as i64;

        let pagination = PaginationInfo {
            current_page: page,
            page_size: per_page,
            total_items: total,
            total_pages,
            has_next_page: page < total_pages,
            has_previous_page: page > 1,
        };

        Ok(PaginatedResponse {
            data: records,
            pagination,
        })
    }
}