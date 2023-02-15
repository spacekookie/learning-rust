use async_std::{
    future::{self, Future},
    sync::{Arc, Mutex},
    task,
};
use std::{collections::BTreeMap, pin::Pin, task::Poll, time::Duration};
use wake_notify::Notify;

type Collection = Arc<Mutex<Notify<BTreeMap<String, String>>>>;

#[async_std::main]
async fn main() {
    let c = Arc::new(Mutex::new(Notify::new(BTreeMap::new())));

    task::spawn(insert_task(Arc::clone(&c)));
    get_task(Arc::clone(&c)).await;
}

async fn insert_task(t: Collection) {
    for i in 0..10 {
        let mut mg = t.lock().await;
        println!("Inserting item...");
        mg.insert(format!("Key {}", i), format!("Value {}", i * 2));
        Notify::wake(&*mg);
        drop(mg);
        task::sleep(Duration::from_millis(1)).await;
    }
}

async fn get_task(t: Collection) {
    let mut i = 0;
    while i < 10 {
        let item = future::poll_fn(|ctx| {
            let mut lock = Box::pin(t.lock());
            match Pin::new(&mut lock).poll(ctx) {
                Poll::Ready(ref mut mg) => match mg.remove(&format!("Key {}", i)) {
                    Some(v) => {
                        i += 1; // Increment for the next key
                        Poll::Ready(v)
                    }
                    None => {
                        println!("Lock acquired but no new item");
                        Notify::add_waker(mg, ctx.waker().clone());
                        Poll::Pending
                    }
                },
                _ => Poll::Pending,
            }
        })
        .await;

        println!("Item: {:?}", item);
    }
}
