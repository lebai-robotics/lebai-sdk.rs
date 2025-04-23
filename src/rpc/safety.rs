use super::Robot;
use cmod::Result;
use proto::google::protobuf::Empty;
use proto::lebai::safety::*;

impl Robot {
    pub(crate) async fn disable_collision_detector(&self) -> Result<()> {
        let req = Empty {};
        let _ = self.c.disable_collision_detector(Some(req)).await.map_err(|e| e.to_string())?;
        Ok(())
    }
    pub(crate) async fn enable_collision_detector(&self) -> Result<()> {
        let req = Empty {};
        let _ = self.c.enable_collision_detector(Some(req)).await.map_err(|e| e.to_string())?;
        Ok(())
    }
    pub(crate) async fn disable_joint_limits(&self) -> Result<()> {
        let req = Empty {};
        let _ = self.c.disable_limit(Some(req)).await.map_err(|e| e.to_string())?;
        Ok(())
    }
    pub(crate) async fn enable_joint_limits(&self) -> Result<()> {
        let req = Empty {};
        let _ = self.c.enable_limit(Some(req)).await.map_err(|e| e.to_string())?;
        Ok(())
    }
}
