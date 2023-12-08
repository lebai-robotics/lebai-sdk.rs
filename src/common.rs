use cmod::Result;
use once_cell::sync::Lazy;
use std::time::{Duration, Instant, SystemTime};

static TIMESTAMP: Lazy<Box<dyn Fn() -> Duration + Send + Sync>> = Lazy::new(|| {
    let time = Instant::now();
    let diff = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
    Box::new(move || time.elapsed() + diff)
});

pub static VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn timestamp() -> Result<u64> {
    Ok(TIMESTAMP().as_millis() as u64)
}

pub async fn sleep_ms(ms: u64) -> Result<()> {
    futures_timer::Delay::new(Duration::from_millis(ms)).await;
    Ok(())
}
