use async_std::task;
use core::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};
use std::time::Duration;

async fn print_in_10_seconds() {
    task::sleep(Duration::from_secs(10)).await;
    println!("It's been like... 5 seconds?")
}

#[async_std::main]
async fn main() {
    let p = print_in_10_seconds();
    
    task::spawn(p);
    task::sleep(Duration::from_secs(2)).await;
    println!("It's been 2 seconds and I'm getting impatient...");
    drop(p);
    task::sleep(Duration::from_secs(5)).await;
}

struct BlockHere;
impl Future for BlockHere {
    type Output = ();
    fn poll(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<()> {
        Poll::Pending
    }
}
