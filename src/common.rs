use cmod::Result;
use once_cell::sync::Lazy;
use std::time::{Duration, Instant, SystemTime};

static TIMESTAMP: Lazy<Box<dyn Fn() -> Duration + Send + Sync>> = Lazy::new(|| {
    let time = Instant::now();
    let diff = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
    Box::new(move || time.elapsed() + diff)
});

pub static VERSION: &'static str = env!("CARGO_PKG_VERSION");

pub fn timestamp() -> Result<u64> {
    Ok(TIMESTAMP().as_millis() as u64)
}

#[cfg(not(target_family = "wasm"))]
pub async fn sleep_ms(ms: u64) -> Result<()> {
    tokio::time::sleep(Duration::from_millis(ms)).await;
    Ok(())
}
