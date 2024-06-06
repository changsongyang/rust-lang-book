use std::{future::Future, time::Duration};

use trpl::Either;

fn main() {
    trpl::block_on(async {
        let slow = async {
            trpl::sleep(Duration::from_secs(5)).await;
            "Finally finished"
        };

        match timeout(Duration::from_secs(2), slow).await {
            Ok(message) => println!("Succeeded with '{message}'"),
            Err(duration) => {
                println!("Failed after {} seconds", duration.as_secs())
            }
        }
    });
}

// ANCHOR: timeout

// ANCHOR: declaration
async fn timeout<F: Future>(
    max_time: Duration,
    future: F,
) -> Result<F::Output, Duration> {
    // ANCHOR_END: declaration
    match trpl::race(future, trpl::sleep(max_time)).await {
        Either::Left(output) => Ok(output),
        Either::Right(_) => Err(max_time),
    }
}
// ANCHOR_END: timeout