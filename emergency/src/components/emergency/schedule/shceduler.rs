use tokio::time::Duration;
use tokio_cron_scheduler::{Job, JobScheduler};

pub async fn start_scheduler() -> Result<(), Box<dyn std::error::Error>> {
    let sched = JobScheduler::new().await?;

    // Add a job that runs every minute
    sched.add(
        Job::new("*/1 * * * * *", |uuid, mut l| {
            println!("Emergency check triggered at {:?}", chrono::Local::now());
       
        })
        ?,
    ).await?;

    sched.start().await?;

    // Keep the scheduler running
    tokio::time::sleep(Duration::MAX).await;

    Ok(())
}
