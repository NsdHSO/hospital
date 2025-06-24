use crate::components::person::PersonService;
use crate::entity::patient::{ActiveModel, Model, PatientRequestBody, PatientWithPerson};
use crate::entity::patient::{Column, Entity};
use crate::entity::person;
use crate::entity::person::PersonRequestBody;
use crate::error_handler::CustomError;
use crate::shared::{PaginatedResponse, PaginationInfo};
use crate::utils::helpers::{check_if_is_duplicate_key_from_data_base, generate_ic, now_time};
use chrono::{Local, NaiveDateTime};
use percent_encoding::percent_decode_str;
use sea_orm::{ActiveModelTrait, Iden, PaginatorTrait, Set};
use sea_orm::{ColumnTrait, QueryFilter};
use sea_orm::{DatabaseConnection, EntityTrait};
use uuid::Uuid;

pub struct PatientService {
    conn: DatabaseConnection,
    person_service: PersonService,
}

impl PatientService {
    pub fn new(conn: &DatabaseConnection) -> Self {
        PatientService {
            conn: conn.clone(),
            person_service: PersonService::new(conn),
        }
    }

    pub(crate) async fn associate_hospital_with_patient(
        &self,
        patient_id: Uuid,
        hospital_id: Uuid,
    ) {
        let query_result = Entity::find()
            .filter(Column::Id.eq(patient_id))
            .one(&self.conn)
            .await;

        match query_result {
            Ok(Some(model)) => {
                let mut query_active: ActiveModel = model.into();
                query_active.updated_at = Set(now_time());
                query_active.hospital_id = Set(Option::from(hospital_id));
                let _ = query_active.update(&self.conn).await;
            }
            Ok(None) => {}
            Err(e) => {
                // Log the error or handle as needed
                eprintln!("Failed to fetch available patient: {}", e);
            }
        }
    }

    pub async fn create_patient(
        &self,
        patient_data: Option<PatientRequestBody>,
    ) -> Result<PatientWithPerson, CustomError> {
        // Check if patient_data exists
        let payload = match patient_data.clone() {
            Some(data) => data,
            None => return Err(CustomError::new(400, "Missing patient data".to_string())),
        };

        let now = Local::now().naive_utc();
        let mut attempts = 0;
        const MAX_ATTEMPTS: usize = 5;
        // Check if patient_ic exists in DB
        let patient_body: PatientRequestBody = patient_data.unwrap();
        let person = self
            .person_service
            .create(Option::from(PersonRequestBody {
                first_name: patient_body.first_name,
                last_name: patient_body.last_name,
                date_of_birth: patient_body.date_of_birth,
                gender: patient_body.gender,
                phone: patient_body.phone,
                email: patient_body.email,
                address: patient_body.address,
                nationality: Some(String::from("ROM")),
                marital_status: None,
                photo_url: None,
            }))
            .await
            .or(Err(CustomError::new(
                500,
                "Internal server error".to_string(),
            )))?;
        if let Some(ref ic) = payload.patient_ic {
            if let Ok(Some(patient)) = self.find_by_field("patient_ic", ic).await {
                return Ok(PatientWithPerson { patient, person });
            }
        }
        loop {
            if attempts >= MAX_ATTEMPTS {
                return Err(CustomError::new(
                    500,
                    "Failed to generate a unique patient IC after multiple attempts.".to_string(),
                ));
            }

            let active_model = Self::generate_model(Some(payload.clone()), now, person.clone().id);

            // Insert the record into the database
            let result = active_model.insert(&self.conn).await;
            if let Some(_) = check_if_is_duplicate_key_from_data_base(&mut attempts, result) {
                let (patient, person) = Entity::find_by_id(person.id)
                    .find_also_related(person::Entity)
                    .one(&self.conn)
                    .await?
                    .ok_or_else(|| CustomError::new(404, "Patient not found".to_string()))?;
                return Ok(PatientWithPerson {
                    patient,
                    person: person.unwrap(),
                });
            }
        }
    }

    /// Find a patient by a given column and value (generic for ic or name)
    pub async fn find_by_field(
        &self,
        field: &str,
        value: &str,
    ) -> Result<Option<Model>, CustomError> {
        let query = match field {
            "patient_ic" => Entity::find().filter(Column::PatientIc.like(value)),
            // "first_name" => Entity::find().filter(Column::FirstName.like(value)),
            _ => {
                return Err(CustomError::new(
                    400,
                    format!("Unsupported field: {}", field),
                ));
            }
        };
        let patient = query
            .one(&self.conn)
            .await
            .map_err(|e| CustomError::new(500, format!("Database error: {}", e)))?;
        if let Some(patient_model) = patient {
            Ok(Some(patient_model))
        } else {
            Err(CustomError::new(
                404,
                format!("Patient not found for {} = '{}'", field, value),
            ))
        }
    }

    #[allow(dead_code)]
    fn parse_filter_value(filter_str: &str, prefix: &str) -> Option<String> {
        if !filter_str.starts_with(prefix) {
            return None;
        }

        let encoded_value = filter_str.strip_prefix(prefix)?;
        percent_decode_str(encoded_value)
            .decode_utf8()
            .map(|v| v.to_string())
            .ok()
            .or_else(|| Some(encoded_value.to_string()))
    }

    #[allow(dead_code)]
    pub async fn find_all(
        &self,
        page: u64,
        per_page: u64,
        filter: Option<String>,
    ) -> Result<PaginatedResponse<Vec<Model>>, CustomError> {
        let mut query = Entity::find();

        if let Some(filter_str) = filter {
            match filter_str.split_once('=') {
                Some(("ic", encoded_name)) => {
                    let patient_ic = percent_decode_str(encoded_name)
                        .decode_utf8()
                        .map(|ic| ic.to_string())
                        .unwrap_or_else(|_| encoded_name.to_string());
                    query = query.filter(Column::PatientIc.like(patient_ic));
                }
                Some(("id", encoded_name)) => {
                    let patient_id = percent_decode_str(encoded_name)
                        .decode_utf8()
                        .map(|id| id.to_string())
                        .unwrap_or_else(|_| encoded_name.to_string());
                    let patient_uuid = Uuid::parse_str(&patient_id)
                        .map_err(|_| CustomError::new(400, "Invalid UUID".to_string()))?;
                    query = query.filter(Column::Id.eq(patient_uuid));
                }
                _ => {}
            }
        }

        let paginator = query.paginate(&self.conn, per_page);
        let total_items = paginator.num_items().await?;
        let total_pages = paginator.num_pages().await?;

        let records = paginator
            .fetch_page(page) // Page is 0-indexed in SeaORM
            .await?;

        let pagination = PaginationInfo {
            current_page: page as i64,
            page_size: per_page as i64,
            total_items: total_items as i64,
            total_pages: total_pages as i64,
            has_next_page: page < total_pages,
            has_previous_page: page > 1,
        };

        Ok(PaginatedResponse {
            data: records,
            pagination,
        })
    }

    /// Associates a patient with a given emergency ID in the emergency_patient table.
    /// If the patient does not exist, it will be created using create_patient.
    /// Returns an error if any association fails.
    pub async fn associate_patient_with_emergency(
        &self,
        emergency_id: Uuid,
        patient_data: Option<PatientRequestBody>,
        transaction: &DatabaseConnection,
    ) -> Result<(), CustomError> {
        use crate::entity::emergency_patient;
        use sea_orm::Set;
        // Create the patient (or you could check if exists first, then create if not)
        let created_patient = self.create_patient(patient_data).await?;
        let junction = emergency_patient::ActiveModel {
            emergency_id: Set(emergency_id),
            patient_id: Set(created_patient.patient.id),
        };
        junction.insert(transaction).await.map_err(|e| {
            CustomError::new(500, format!("Failed to link patient to emergency: {}", e))
        })?;

        Ok(())
    }

    /// Associates a list of patients with a given emergency ID in the emergency_patient table.
    /// If a patient does not exist, it will be created using create_patient.
    /// Returns an error if any association fails.
    pub async fn associate_patients_with_emergency(
        &self,
        emergency_id: Uuid,
        patients: &[PatientRequestBody],
        transaction: &DatabaseConnection,
    ) -> Result<(), CustomError> {
        for patient_data in patients {
            self.associate_patient_with_emergency(
                emergency_id,
                Some(patient_data.clone()),
                transaction,
            )
            .await?;
        }
        Ok(())
    }

    /// Find all patients related to a given emergency ID (many-to-many).
    pub async fn find_patients_by_emergency_id(
        &self,
        emergency_id: Uuid,
    ) -> Result<Vec<Model>, CustomError> {
        use crate::entity::{emergency_patient, patient};
        let patient_models = emergency_patient::Entity::find()
            .filter(emergency_patient::Column::EmergencyId.eq(emergency_id))
            .find_also_related(patient::Entity)
            .all(&self.conn)
            .await
            .map_err(|e| CustomError::new(500, format!("Database error: {}", e)))?;
        Ok(patient_models.into_iter().filter_map(|(_, p)| p).collect())
    }

    /// Partially updates a patient by UUID. Only provided fields are updated.
    #[allow(dead_code)]
    pub async fn update_patient(
        &self,
        uuid: Uuid,
        payload: PatientRequestBody,
    ) -> Result<Model, CustomError> {
        use sea_orm::Set;
        let now = now_time();
        let patient = Entity::find()
            .filter(Column::Id.eq(uuid))
            .one(&self.conn)
            .await?;
        let model = match patient {
            Some(model) => model,
            None => {
                return Err(CustomError::new(404, "Patient not found".to_string()));
            }
        };
        let mut active_model: ActiveModel = model.into();

        if let Some(val) = payload.patient_ic {
            active_model.patient_ic = Set(Some(val));
        }
        if let Some(val) = payload.hospital_id {
            active_model.hospital_id = Set(Option::from(val));
        }
        if let Some(val) = payload.emergency_contact {
            active_model.emergency_contact = Set(Some(val));
        }
        if let Some(val) = payload.blood_type {
            active_model.blood_type = Set(Some(val));
        }
        if let Some(val) = payload.allergies {
            active_model.allergies = Set(Some(val));
        }
        if let Some(val) = payload.medical_history {
            active_model.medical_history = Set(Some(val));
        }

        active_model.updated_at = Set(now);
        let updated = active_model
            .update(&self.conn)
            .await
            .map_err(|e| CustomError::new(500, format!("Database error: {}", e)))?;
        Ok(updated)
    }

    fn generate_model(
        p0: Option<PatientRequestBody>,
        p1: NaiveDateTime,
        person_id: Uuid,
    ) -> ActiveModel {
        let payload = p0.unwrap_or_default();
        ActiveModel {
            patient_ic: Set(Some(generate_ic().to_string())),
            hospital_id: if let Some(value) = payload.hospital_id {
                Set(Option::from(value))
            } else {
                Set(Default::default())
            },
            created_at: Set(p1),
            updated_at: Set(p1),
            emergency_contact: if let Some(value) = payload.emergency_contact {
                Set(Some(value))
            } else {
                Set(Default::default())
            },
            blood_type: if let Some(value) = payload.blood_type {
                Set(Some(value))
            } else {
                Set(Default::default())
            },
            allergies: if let Some(value) = payload.allergies {
                Set(Some(value))
            } else {
                Set(Default::default())
            },
            medical_history: if let Some(value) = payload.medical_history {
                Set(Some(value))
            } else {
                Set(Default::default())
            },
            id: Set(person_id),
        }
    }
}
