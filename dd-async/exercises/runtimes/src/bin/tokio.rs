use tokio::{runtime::Runtime, task};

async fn hello() {
    println!("Hello world!");
}

fn main() {
    let rt = Runtime::new().unwrap();
    task::block_in_place(move || {
        let local = task::LocalSet::new();
        local.block_on(&rt, hello());
    })
}
