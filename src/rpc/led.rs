use super::Robot;
use cmod::Result;
use proto::lebai::db::LoadRequest;
use proto::lebai::led::*;
use proto::led::LedStyle;

impl Robot {
    pub(crate) async fn load_led_style(&self, name: String, dir: Option<String>) -> Result<LedStyle> {
        let req = LoadRequest {
            name,
            dir: dir.unwrap_or_default(),
        };
        let resp = self.c.load_led_style(Some(req)).await.map_err(|e| e.to_string())?;
        Ok(resp.into())
    }
    pub(crate) async fn set_led_style(&self, style: LedStyle) -> Result<()> {
        self.set_led(style.mode, style.speed, style.colors).await?;
        self.set_voice(style.voice, style.volume).await?;
        Ok(())
    }
    pub(crate) async fn set_led(&self, mode: i32, speed: i32, colors: Vec<i32>) -> Result<()> {
        let req = LedData { mode, speed, colors };
        let _ = self.c.set_led(Some(req)).await.map_err(|e| e.to_string())?;
        Ok(())
    }
    pub(crate) async fn set_voice(&self, voice: i32, volume: i32) -> Result<()> {
        let req = VoiceData { voice, volume };
        let _ = self.c.set_voice(Some(req)).await.map_err(|e| e.to_string())?;
        Ok(())
    }
    pub(crate) async fn set_fan(&self, mode: i32) -> Result<()> {
        let req = FanData { mode };
        let _ = self.c.set_fan(Some(req)).await.map_err(|e| e.to_string())?;
        Ok(())
    }
}
