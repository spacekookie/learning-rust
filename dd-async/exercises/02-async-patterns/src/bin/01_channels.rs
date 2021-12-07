use async_std::{channel, task};
use std::time::Duration;

#[async_std::main]
async fn main() {
    let (tx, rx) = channel::unbounded();

    task::spawn(async move {
        for i in 0..3 {
            task::sleep(Duration::from_secs(1)).await;
            tx.send(i as i64).await;
        }
    });

    while let Ok(i) = rx.recv().await {
        println!("Received message: {}", i);
    }
}
