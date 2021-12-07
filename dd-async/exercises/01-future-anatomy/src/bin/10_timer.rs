use std::time::{Duration, Instant};

struct Timer {
    then: Instant,
    len: Duration,
}

#[async_std::main]
async fn main() {
    println!("Hello world!");
    Timer::new(Duration::from_secs(1)).await;
    println!("This is one second later!");
}
