#[tokio::main]
async fn main() {
    #[cfg(feature = "mdns")]
    println!("{:?}", lebai_sdk::discover_devices(5.0).await.unwrap());
    core::future::pending().await
}
