use super::Robot;
use cmod::Result;
use proto::lebai::auto::*;

impl Robot {
    pub(crate) async fn set_auto(&self, name: AutoCfg, value: bool) -> Result<()> {
        let req = SetAutoRequest { name: name as i32, value };
        let _ = self.c.set_auto(Some(req)).await.map_err(|e| e.to_string())?;
        Ok(())
    }
    pub(crate) async fn get_auto(&self, name: AutoCfg) -> Result<Option<bool>> {
        let req = GetAutoRequest { name: name as i32 };
        let resp = self.c.get_auto(Some(req)).await.map_err(|e| e.to_string())?;
        Ok(resp.value)
    }
}
