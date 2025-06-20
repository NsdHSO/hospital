use sea_orm::{DatabaseConnection, DbErr};

pub struct PersonService {
    db: DatabaseConnection,
}

impl PersonService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}
