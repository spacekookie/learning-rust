use async_std::{channel, task};
use std::time::Duration;

#[async_std::main]
async fn main() {
    let (tx, rx) = channel::unbounded();

    task::spawn(async move {
        for _ in 0..100 {
            tx.send((0..100_000).map(|i| i as u8).collect::<Vec<u8>>())
                .await
                .unwrap();
        }
    });

    while let Ok(i) = rx.recv().await {
        println!("Received message: {:?}", i.first());
        task::sleep(Duration::from_millis(100)).await;
    }
}
