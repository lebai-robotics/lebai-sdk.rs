use super::Robot;
use cmod::Result;
use proto::google::protobuf::Empty;
use proto::lebai::system::*;

impl Robot {
    pub(crate) async fn start_sys(&self) -> Result<()> {
        let _ = self.c.start_sys(Some(Empty {})).await.map_err(|e| e.to_string());
        Ok(())
    }
    pub(crate) async fn stop_sys(&self) -> Result<()> {
        let _ = self.c.stop_sys(Some(Empty {})).await.map_err(|e| e.to_string());
        Ok(())
    }
    pub(crate) async fn powerdown(&self) -> Result<()> {
        let _ = self.c.powerdown(Some(Empty {})).await.map_err(|e| e.to_string());
        Ok(())
    }
    pub(crate) async fn reboot(&self) -> Result<()> {
        let _ = self.c.reboot(Some(Empty {})).await.map_err(|e| e.to_string());
        Ok(())
    }
    pub(crate) async fn stop(&self) -> Result<()> {
        let _ = self.c.stop(Some(Empty {})).await.map_err(|e| e.to_string());
        Ok(())
    }
    pub(crate) async fn estop(&self) -> Result<()> {
        let _ = self.c.estop(Some(Empty {})).await.map_err(|e| e.to_string());
        Ok(())
    }
}
