use super::Robot;
use cmod::Result;
use proto::google::protobuf::Empty;
use proto::lebai::claw::*;

impl Robot {
    pub(crate) async fn init_claw(&self, force: Option<bool>) -> Result<()> {
        let req = InitClawRequest {
            force: force.unwrap_or_default(),
        };
        let _ = self.c.init_claw(Some(req)).await.map_err(|e| e.to_string())?;
        Ok(())
    }
    pub(crate) async fn set_claw(&self, force: Option<f64>, amplitude: Option<f64>) -> Result<()> {
        let req = SetClawRequest {
            force: force.map(|x| x.into()),
            amplitude: amplitude.map(|x| x.into()),
        };
        let _ = self.c.set_claw(Some(req)).await.map_err(|e| e.to_string())?;
        Ok(())
    }
    pub(crate) async fn get_claw(&self) -> Result<Claw> {
        let resp = self.c.get_claw(Some(Empty {})).await.map_err(|e| e.to_string())?;
        Ok(resp)
    }
}
