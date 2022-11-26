use super::Robot;
use cmod::Result;
use proto::lebai::motion::*;
use proto::posture::Pose;

impl Robot {
    pub async fn movej(&self, p: Pose, a: f64, v: f64, t: f64, r: Option<f64>) -> Result<u32> {
        let param = MoveParam {
            acc: a,
            velocity: v,
            time: t,
            radius: r.unwrap_or_default(),
        };
        let req = MoveRequest {
            pose: Some(p.into()),
            param: Some(param),
        };
        let id = self.c.move_joint(Some(req)).await.map_err(|e| e.to_string())?;
        Ok(id.id)
    }
    pub async fn movel(&self, p: Pose, a: f64, v: f64, t: f64, r: Option<f64>) -> Result<u32> {
        let param = MoveParam {
            acc: a,
            velocity: v,
            time: t,
            radius: r.unwrap_or_default(),
        };
        let req = MoveRequest {
            pose: Some(p.into()),
            param: Some(param),
        };
        let id = self.c.move_linear(Some(req)).await.map_err(|e| e.to_string())?;
        Ok(id.id)
    }
}
