use future_anatomy::never::*;

#[async_std::main]
async fn main() {
    let _ = MyFuture.await; // blocks forever
}
