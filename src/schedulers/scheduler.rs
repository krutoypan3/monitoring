use std::thread::sleep;
use std::time::Duration;

pub async fn start() {
    println!("Scheduler is starting...");
    loop {
        println!("Current time is {:?}", chrono::Utc::now().format("%H:%M:%S").to_string());
        sleep(Duration::from_secs(1));
    }
}