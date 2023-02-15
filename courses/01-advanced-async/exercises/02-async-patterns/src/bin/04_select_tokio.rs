use std::time::Duration;
use tokio::{select, time::sleep};

async fn quickly() -> String {
    "Available immediately!".into()
}

async fn slowly() -> String {
    println!("Running slowly...");
    sleep(Duration::from_secs(1)).await;
    "Available slowly...".into()
}

#[tokio::main]
async fn main() {
    println!(
        "Msg: {}",
        select! {
            s = slowly() => s,
            s = quickly() => s,
        }
    );
}
