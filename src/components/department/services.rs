use sea_orm::{DatabaseConnection, DbErr};

pub struct DepartmentService {
    db: DatabaseConnection,
}

impl DepartmentService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}
