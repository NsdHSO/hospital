use sea_orm::{DatabaseConnection, DbErr};

pub struct StaffService {
    db: DatabaseConnection,
}

impl StaffService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}
