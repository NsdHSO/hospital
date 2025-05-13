use crate::components::emergency::schedule::emergency_allocation::EmergencyAllocationService;
use actix_web::web;
use log::{error, warn};
use sea_orm::DatabaseConnection;
use std::sync::atomic::{AtomicBool, Ordering};
use tokio::time::{interval, Duration};
// Create a static flag to track if a process is running
static ALLOCATION_RUNNING: AtomicBool = AtomicBool::new(false);

pub async fn start_scheduler(
    db_conn: &DatabaseConnection,
) -> Result<(), Box<dyn std::error::Error>> {
    // Define the interval at which the task should run
    let mut interval = interval(Duration::from_secs(30*10));

    println!("Emergency allocation scheduler started");

    loop {
        interval.tick().await;
        println!("Emergency check triggered at {:?}", chrono::Local::now());

        // Check if another allocation process is already running
        if ALLOCATION_RUNNING
            .compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst)
            .is_ok()
        {
            // We successfully set the flag to true, meaning we can run the process
            println!("Starting new emergency allocation process");

            let service = EmergencyAllocationService::new(db_conn);

            // Run the allocation process
            match service.run_allocation_process().await {
                Ok(_) => println!("Emergency allocation process completed successfully"),
                Err(e) => error!("Emergency allocation process failed: {}", e),
            }

            // Reset the flag when done
            ALLOCATION_RUNNING.store(false, Ordering::SeqCst);
        } else {
            // Another process is already running
            warn!("Skipping allocation - previous process still running");
        }
    }
}
