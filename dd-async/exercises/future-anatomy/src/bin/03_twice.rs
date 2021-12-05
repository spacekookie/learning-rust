use future_anatomy::good_twice::*;

#[async_std::main]
async fn main() {
    println!("Future says: {:?}", Twice::new().await)
}
