use chrono::Utc;
use log::{error, info, warn};
use sea_orm::*;
use uuid::Uuid;

use crate::entity::sea_orm_active_enums::{AmbulanceStatusEnum, EmergencyStatusEnum};
use crate::entity::{ambulance, emergency};
use crate::error_handler::CustomError;
use crate::utils::utils::calculate_distance;

pub struct EmergencyAllocationService {
    conn: DatabaseConnection,
}

impl EmergencyAllocationService {
    pub fn new(conn: &DatabaseConnection) -> Self {
        Self { conn: conn.clone() }
    }

    fn clone(&self) -> Self {
        Self {
            conn: self.conn.clone(),
        }
    }

    pub async fn run_allocation_process(&self) -> Result<(), CustomError> {
        println!("Starting emergency allocation process");

        let this = self.clone();
        self.conn
            .transaction(|txn| {
                Box::pin(async move { this.allocate_emergencies_in_transaction(txn).await })
            })
            .await
            .map_err(|e| {
                error!("Error during emergency allocation transaction: {}", e);
                CustomError::new(500, format!("Transaction failed: {}", e))
            })
    }

    async fn allocate_emergencies_in_transaction(
        &self,
        txn: &DatabaseTransaction,
    ) -> Result<(), CustomError> {
        let pending_emergencies = Self::fetch_pending_emergencies(txn).await?;
        if pending_emergencies.is_empty() {
            println!("No pending emergencies found for allocation");
            return Ok(());
        }

        println!(
            "Found {} pending emergencies for allocation",
            pending_emergencies.len()
        );

        let available_ambulances = Self::fetch_available_ambulances(txn).await?;
        if available_ambulances.is_empty() {
            warn!("No available ambulances found for allocation");
            return Ok(());
        }

        println!("Found {} available ambulances", available_ambulances.len());

        let mut available_ambulance_ids: Vec<Uuid> =
            available_ambulances.iter().map(|a| a.id).collect();
        let emergency_count = pending_emergencies.len();
        for (index, emergency) in pending_emergencies.into_iter().enumerate() {
            if available_ambulance_ids.is_empty() {
                println!(
                    "No more ambulances available for allocation. Remaining pending emergencies: {}",
                    emergency_count - index
                );
                break;
            }

            let lat = emergency
                .emergency_latitude
                .to_string()
                .parse::<f64>()
                .unwrap_or(0.0);
            let lon = emergency
                .emergency_longitude
                .to_string()
                .parse::<f64>()
                .unwrap_or(0.0);

            if let Some(idx) = Self::find_closest_ambulance_index(lat, lon, &available_ambulances) {
                let ambulance_id = available_ambulances[idx].id;

                if let Some(_) = available_ambulance_ids
                    .iter()
                    .position(|id| *id == ambulance_id)
                {
                    let ambulance = &available_ambulances[idx];

                    match dispatch_ambulance(txn, ambulance, &emergency).await {
                        Ok(_) => {
                            println!(
                                "Successfully dispatched ambulance {} to emergency {}",
                                ambulance.id, emergency.id
                            );
                            available_ambulance_ids.retain(|id| *id != ambulance_id);
                        }
                        Err(e) => {
                            error!("Failed to dispatch ambulance: {}", e);
                            return Err(e);
                        }
                    }
                }
            }
        }

        println!("Emergency allocation process completed within transaction");
        Ok(())
    }

    async fn fetch_pending_emergencies(
        txn: &DatabaseTransaction,
    ) -> Result<Vec<emergency::Model>, CustomError> {
        emergency::Entity::find()
            .filter(emergency::Column::Status.eq(EmergencyStatusEnum::Pending))
            .order_by_desc(emergency::Column::Severity)
            .order_by_asc(emergency::Column::CreatedAt)
            .limit(1000)
            .all(txn)
            .await
            .map_err(|e| {
                CustomError::new(500, format!("Failed to fetch pending emergencies: {}", e))
            })
    }

    async fn fetch_available_ambulances(
        txn: &DatabaseTransaction,
    ) -> Result<Vec<ambulance::Model>, CustomError> {
        ambulance::Entity::find()
            .filter(ambulance::Column::Status.eq(AmbulanceStatusEnum::Available))
            .limit(1000)
            .all(txn)
            .await
            .map_err(|e| {
                CustomError::new(500, format!("Failed to fetch available ambulances: {}", e))
            })
    }

    fn find_closest_ambulance_index(
        emergency_lat: f64,
        emergency_lon: f64,
        ambulances: &[ambulance::Model],
    ) -> Option<usize> {
        if ambulances.is_empty() {
            return None;
        }

        let mut closest_index = 0;
        let mut shortest_distance = calculate_distance(
            emergency_lat,
            emergency_lon,
            ambulances[0]
                .location_latitude
                .to_string()
                .parse::<f64>()
                .unwrap_or(0.0),
            ambulances[0]
                .location_longitude
                .to_string()
                .parse::<f64>()
                .unwrap_or(0.0),
        );

        for (i, a) in ambulances.iter().enumerate().skip(1) {
            let lat = a
                .location_latitude
                .to_string()
                .parse::<f64>()
                .unwrap_or(0.0);
            let lon = a
                .location_longitude
                .to_string()
                .parse::<f64>()
                .unwrap_or(0.0);
            let distance = calculate_distance(emergency_lat, emergency_lon, lat, lon);

            if distance < shortest_distance {
                shortest_distance = distance;
                closest_index = i;
            }
        }

        Some(closest_index)
    }
}
async fn dispatch_ambulance(
    txn: &DatabaseTransaction,
    ambulance: &ambulance::Model,
    emergency: &emergency::Model,
) -> Result<(), CustomError> {
    println!(
        "Starting dispatch_ambulance function for emergency: {} and ambulance: {}",
        emergency.id, ambulance.id
    );

    // Log the initial state
    println!(
        "Initial Emergency State - ID: {}, Status: {:?}, Ambulance ID: {:?}",
        emergency.id, emergency.status, emergency.id_ambulance
    );
    println!(
        "Initial Ambulance State - ID: {}, Status: {:?}",
        ambulance.id, ambulance.status
    );

    // Create the emergency active model for updating
    let mut emergency_active_model: emergency::ActiveModel = emergency.clone().into();
    emergency_active_model.status = Set(EmergencyStatusEnum::InProgress);
    emergency_active_model.id_ambulance = Set(Some(ambulance.id));
    emergency_active_model.updated_at = Set(Utc::now().naive_utc());

    println!(
        "Attempting to update emergency with status: {:?} and ambulance_id: {:?}",
        EmergencyStatusEnum::InProgress,
        ambulance.id
    );

    // Update emergency and log the result
    match emergency_active_model.update(txn).await {
        Ok(updated) => {
            println!(
                "Emergency update SUCCESS - ID: {}, New Status: {:?}, Ambulance ID: {:?}",
                emergency.id,
                updated.status.clone(),
                updated.id_ambulance.clone().unwrap_or_default()
            );
        }
        Err(e) => {
            error!(
                "Emergency update FAILED - ID: {}, Error: {}",
                emergency.id, e
            );
            return Err(CustomError::new(
                500,
                format!(
                    "Failed to update emergency status for {}: {}",
                    emergency.id, e
                ),
            ));
        }
    }

    let mut ambulance_active_model: ambulance::ActiveModel = ambulance.clone().into();
    ambulance_active_model.status = Set(AmbulanceStatusEnum::Dispatched);
    ambulance_active_model.updated_at = Set(Utc::now().naive_utc());

    println!(
        "Attempting to update ambulance with status: {:?}",
        AmbulanceStatusEnum::Dispatched
    );

    match ambulance_active_model.update(txn).await {
        Ok(updated) => {
            println!(
                "Ambulance update SUCCESS - ID: {}, New Status: {:?}",
                ambulance.id,
                updated.status.clone()
            );
        }
        Err(e) => {
            error!(
                "Ambulance update FAILED - ID: {}, Error: {}",
                ambulance.id, e
            );
            return Err(CustomError::new(
                500,
                format!(
                    "Failed to update ambulance status for {}: {}",
                    ambulance.id, e
                ),
            ));
        }
    }
    println!(
        "dispatch_ambulance function COMPLETED SUCCESSFULLY for emergency: {} and ambulance: {}",
        emergency.id, ambulance.id
    );
    Ok(())
}