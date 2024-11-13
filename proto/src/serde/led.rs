use crate::lebai::led;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct LedStyle {
    pub mode: i32,
    pub speed: i32,
    pub colors: Vec<i32>,
    pub voice: i32,
    pub volume: i32,
}
impl From<led::LedStyle> for LedStyle {
    fn from(style: led::LedStyle) -> Self {
        let led = style.led.unwrap_or_default();
        Self {
            mode: led.mode,
            speed: led.speed,
            colors: led.colors,
            voice: style.voice,
            volume: style.volume,
        }
    }
}
