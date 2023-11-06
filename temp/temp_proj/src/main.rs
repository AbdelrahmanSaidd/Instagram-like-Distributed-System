use std::time::Duration;
use tokio::time::{self, Duration as TokioDuration};
use tokio::select;

#[tokio::main]
async fn main() {
    // Define the timeout duration
    let timeout_duration = Duration::from_secs(3);

    // Perform some asynchronous operation (e.g., a future that sleeps for 3 seconds)
    let async_operation = async {
        // Simulate an asynchronous operation
        tokio::time::sleep(TokioDuration::from_secs(5)).await;
        println!("Async operation completed.");
    };

    // Use `select!` to combine the asynchronous operation and the timeout
    select! {
        _ = async_operation => {
            // The asynchronous operation completed before the timeout.
            println!("Async operation completed within the timeout.");
        }
        _ = time::sleep(timeout_duration) => {
            // The timeout occurred.
            println!("Timeout occurred. Async operation did not complete in time.");
        }
    }
}
