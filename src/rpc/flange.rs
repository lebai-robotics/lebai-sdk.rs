use super::Robot;
use cmod::Result;
use proto::lebai::flange::*;

impl Robot {
    pub(crate) async fn set_flange_baud_rate(&self, baud_rate: u32) -> Result<()> {
        let req = SetFlangeBaudRateRequest { baud_rate };
        let _ = self.c.set_flange_baud_rate(Some(req)).await.map_err(|e| e.to_string())?;
        Ok(())
    }
}
