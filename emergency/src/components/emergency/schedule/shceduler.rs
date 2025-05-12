use tokio::time::{interval, Duration};
// Remove the import for Job and JobScheduler

pub async fn start_scheduler() -> Result<(), Box<dyn std::error::Error>> {
    // Define the interval at which the task should run (e.g., every 10 minute)
    let mut interval = interval(Duration::from_secs(600));

    loop {
        interval.tick().await;

        println!("Emergency check triggered at {:?}", chrono::Local::now());
    }
}
