use crate::components::emergency::services::EmergencyService;
use actix_web::web;
use sea_orm::DatabaseConnection;
use tokio::time::{interval, Duration};
use crate::components::emergency::schedule::emergency_allocation::EmergencyAllocationService;
// Remove the import for Job and JobScheduler

pub async fn start_scheduler(
    db_conn: &DatabaseConnection,
) -> Result<(), Box<dyn std::error::Error>> {
    // Define the interval at which the task should run (e.g., every 10 minute)
    let mut interval = interval(Duration::from_secs(60*10));

    loop {
        interval.tick().await;
        let service = EmergencyAllocationService::new(db_conn);
        service
            .run_allocation_process()
            .await
            .expect("TODO: panic message");
        println!("Emergency check triggered at {:?}", chrono::Local::now());
    }
}
