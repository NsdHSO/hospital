use crate::components::emergency::schedule::emergency_allocation::EmergencyAllocationService;
use log::{error, warn};
use sea_orm::DatabaseConnection;
use std::sync::atomic::{AtomicBool, Ordering};
use tokio::time::Duration;
// Create a static flag to track if a process is running
static ALLOCATION_RUNNING: AtomicBool = AtomicBool::new(false);
pub async fn start_scheduler(
    db_conn: &DatabaseConnection,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("Emergency allocation scheduler started");

    loop {
        let now = chrono::Local::now();
        println!("Emergency check triggered at {now:?}");

        if ALLOCATION_RUNNING
            .compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst)
            .is_ok()
        {
            println!("Starting new emergency allocation process");

            let service = EmergencyAllocationService::new(db_conn);

            let result = service.run_allocation_process().await;

            match result {
                Ok(_) => println!("Emergency allocation process completed successfully"),
                Err(e) => error!("Emergency allocation process failed: {e}"),
            }

            ALLOCATION_RUNNING.store(false, Ordering::SeqCst);
        } else {
            warn!("Skipping allocation - previous process still running");
        }

        println!("Sleeping for 1 hour...");
        tokio::time::sleep(Duration::from_secs(60 * 60)).await;
    }
}
