use super::Robot;
use cmod::Result;
use proto::lebai::signal::*;

impl Robot {
    pub(crate) async fn set_signal(&self, index: u32, value: i32) -> Result<()> {
        let req = SetSignalRequest { key: index, value };
        let _ = self.c.set_signal(Some(req)).await.map_err(|e| e.to_string())?;
        Ok(())
    }
    pub(crate) async fn get_signal(&self, index: u32) -> Result<i32> {
        let req = GetSignalRequest { key: index };
        let resp = self.c.get_signal(Some(req)).await.map_err(|e| e.to_string())?;
        Ok(resp.value)
    }
    pub(crate) async fn add_signal(&self, index: u32, value: i32) -> Result<()> {
        let req = SetSignalRequest { key: index, value };
        let _ = self.c.add_signal(Some(req)).await.map_err(|e| e.to_string())?;
        Ok(())
    }
}
