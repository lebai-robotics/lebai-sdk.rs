use super::Robot;
use cmod::Result;
use proto::google::protobuf::Empty;
use proto::kinematic::KinData;
use proto::lebai::db::LoadRequest;
use proto::lebai::kinematic::*;
use proto::posture::CartesianPose;

impl Robot {
    pub(crate) async fn load_tcp(&self, name: String, dir: Option<String>) -> Result<CartesianPose> {
        let req = LoadRequest {
            name,
            dir: dir.unwrap_or_default(),
        };
        let resp = self.c.load_tcp(Some(req)).await.map_err(|e| e.to_string())?;
        Ok(resp.into())
    }
    pub(crate) async fn set_tcp(&self, pose: CartesianPose) -> Result<()> {
        let _ = self.c.set_tcp(Some(pose.into())).await.map_err(|e| e.to_string())?;
        Ok(())
    }
    pub(crate) async fn get_tcp(&self) -> Result<CartesianPose> {
        let resp = self.c.get_tcp(Some(Empty {})).await.map_err(|e| e.to_string())?;
        Ok(resp.into())
    }
    pub(crate) async fn set_velocity_factor(&self, speed_factor: i32) -> Result<()> {
        let req = KinFactor { speed_factor };
        let _ = self.c.set_kin_factor(Some(req)).await.map_err(|e| e.to_string())?;
        Ok(())
    }
    pub(crate) async fn get_velocity_factor(&self) -> Result<i32> {
        let resp = self.c.get_kin_factor(Some(Empty {})).await.map_err(|e| e.to_string())?;
        Ok(resp.speed_factor)
    }
    pub(crate) async fn get_kin_data(&self) -> Result<KinData> {
        let resp = self.c.get_kin_data(Some(Empty {})).await.map_err(|e| e.to_string())?;
        Ok(resp.into())
    }
}
