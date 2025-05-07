use crate::ambulance::models::Ambulance;
use crate::db::config;
use crate::db::config::DbConnection;
use crate::error_handler::CustomError;
use crate::shared::{PaginatedResponse, PaginationInfo};
use diesel::dsl::sql;
use diesel::sql_types::BigInt;
use diesel::RunQueryDsl;
use diesel::{QueryDsl, SelectableHelper};

pub struct AmbulanceService {
    conn: DbConnection,
}

impl AmbulanceService {
    pub fn new() -> Result<Self, CustomError> {
        let conn = config::connection()?;
        Ok(AmbulanceService { conn })
    }
    pub fn find_all(
        &mut self,
        page: i64,
        per_page: i64,
    ) -> Result<PaginatedResponse<Vec<Ambulance>>, CustomError> {
        use crate::schema::ambulance::dsl::*;

        let offset = (page - 1) * per_page;

        // Single query that gets both count and records
        let records_with_count: Vec<(Ambulance, i64)> = ambulance
            .limit(per_page)
            .offset(offset)
            .select((Ambulance::as_select(), sql::<BigInt>("COUNT(*) OVER()")))
            .load(&mut self.conn)?;

        let total = records_with_count
            .first()
            .map(|(_, count)| count)
            .unwrap_or(&0)
            .to_owned();
        let records = records_with_count
            .into_iter()
            .map(|(record, _)| record)
            .collect();
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
