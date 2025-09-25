use crate::entity::auth_identity;
use crate::http_response::error_handler::CustomError;
use crate::http_response::HttpCodeW;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, Set};
use uuid::Uuid;

pub struct AuthIdentityService {
    conn: DatabaseConnection,
}

impl AuthIdentityService {
    pub fn new(conn: &DatabaseConnection) -> Self {
        Self { conn: conn.clone() }
    }

    pub async fn find_by_user_sub(
        &self,
        user_sub: &str,
    ) -> Result<Option<auth_identity::Model>, CustomError> {
        let rec = auth_identity::Entity::find_by_id(user_sub.to_string())
            .one(&self.conn)
            .await
            .map_err(|e| CustomError::new(HttpCodeW::InternalServerError, format!("DB error: {e}")))?;
        Ok(rec)
    }

    pub async fn upsert_mapping(&self, user_sub: &str, person_id: Uuid) -> Result<(), CustomError> {
        if let Some(existing) = self
            .find_by_user_sub(user_sub)
            .await?
        {
            let mut act: auth_identity::ActiveModel = existing.into();
            act.person_id = Set(person_id);
            act.update(&self.conn)
                .await
                .map_err(|e| CustomError::new(HttpCodeW::InternalServerError, format!("DB update error: {e}")))?;
            Ok(())
        } else {
            let act = auth_identity::ActiveModel {
                user_sub: Set(user_sub.to_string()),
                person_id: Set(person_id),
                created_at: Set(chrono::Utc::now().naive_utc()),
            };
            act.insert(&self.conn)
                .await
                .map_err(|e| CustomError::new(HttpCodeW::InternalServerError, format!("DB insert error: {e}")))?;
            Ok(())
        }
    }
}