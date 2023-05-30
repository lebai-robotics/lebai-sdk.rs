use super::Robot;
use cmod::Result;
use proto::lebai::signal::*;

impl Robot {
    pub(crate) async fn set_signal(&self, index: u32, value: i32) -> Result<()> {
        let req = SetSignalRequest { key: index, value };
        let _ = self.c.set_signal(Some(req)).await.map_err(|e| e.to_string())?;
        Ok(())
    }
    pub(crate) async fn set_signals(&self, index: u32, values: Vec<i32>) -> Result<()> {
        let req = SetSignalsRequest { key: index, values };
        let _ = self.c.set_signals(Some(req)).await.map_err(|e| e.to_string())?;
        Ok(())
    }
    pub(crate) async fn get_signal(&self, index: u32) -> Result<i32> {
        let req = GetSignalRequest { key: index };
        let resp = self.c.get_signal(Some(req)).await.map_err(|e| e.to_string())?;
        Ok(resp.value)
    }
    pub(crate) async fn get_signals(&self, index: u32, len: u32) -> Result<Vec<i32>> {
        let req = GetSignalsRequest { key: index, len };
        let resp = self.c.get_signals(Some(req)).await.map_err(|e| e.to_string())?;
        Ok(resp.values)
    }
    pub(crate) async fn add_signal(&self, index: u32, value: i32) -> Result<()> {
        let req = SetSignalRequest { key: index, value };
        let _ = self.c.add_signal(Some(req)).await.map_err(|e| e.to_string())?;
        Ok(())
    }
}
