use future_anatomy::twice::*;

#[async_std::main]
async fn main() {
    Twice::new().await
}
