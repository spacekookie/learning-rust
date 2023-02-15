use async_std::task;

async fn hello() {
    println!("Hello world!");
}

fn main() {
    task::block_on(hello());
}
