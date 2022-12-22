use super::Robot;
use cmod::Result;
use proto::google::protobuf::Empty;
use proto::lebai::claw::*;

impl Robot {
    pub(crate) async fn set_claw(&self, force: f64, amplitude: f64) -> Result<()> {
        let req = SetClawRequest {
            force: Some(force.into()),
            amplitude: Some(amplitude.into()),
        };
        let _ = self.c.set_claw(Some(req)).await.map_err(|e| e.to_string())?;
        Ok(())
    }
    pub(crate) async fn get_claw(&self) -> Result<(f64, f64, f64, bool)> {
        let resp = self.c.get_claw(Some(Empty {})).await.map_err(|e| e.to_string())?;
        Ok((resp.force, resp.amplitude, resp.weight, resp.hold_on))
    }
}
