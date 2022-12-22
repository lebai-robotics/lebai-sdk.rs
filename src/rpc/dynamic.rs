use super::Robot;
use cmod::Result;
use proto::google::protobuf::Empty;
use proto::lebai::db::LoadRequest;
use proto::lebai::dynamic::*;
use proto::lebai::kinematic::*;
use proto::lebai::posture::{CartesianPose, Position};

impl Robot {
    pub(crate) async fn load_tcp(&self, name: String, dir: String) -> Result<CartesianPose> {
        let req = LoadRequest { name, dir };
        let resp = self.c.load_tcp(Some(req)).await.map_err(|e| e.to_string())?;
        Ok(resp)
    }
    pub(crate) async fn set_tcp(&self, pose: CartesianPose) -> Result<()> {
        let _ = self.c.set_tcp(Some(pose)).await.map_err(|e| e.to_string())?;
        Ok(())
    }
    pub(crate) async fn get_tcp(&self) -> Result<CartesianPose> {
        let resp = self.c.get_tcp(Some(Empty {})).await.map_err(|e| e.to_string())?;
        Ok(resp)
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
    pub(crate) async fn load_payload(&self, name: String, dir: String) -> Result<(f64, Option<Position>)> {
        let req = LoadRequest { name, dir };
        let resp = self.c.load_payload(Some(req)).await.map_err(|e| e.to_string())?;
        Ok((resp.mass, resp.cog))
    }
    pub(crate) async fn set_payload(&self, mass: f64, cog: Position) -> Result<()> {
        let req = SetPayloadRequest {
            mass: Some(mass.into()),
            cog: Some(cog),
        };
        let _ = self.c.set_payload(Some(req)).await.map_err(|e| e.to_string())?;
        Ok(())
    }
    pub(crate) async fn get_payload(&self) -> Result<(f64, Option<Position>)> {
        let resp = self.c.get_payload(Some(Empty {})).await.map_err(|e| e.to_string())?;
        Ok((resp.mass, resp.cog))
    }
    pub(crate) async fn set_gravity(&self, pose: Position) -> Result<()> {
        let _ = self.c.set_gravity(Some(pose)).await.map_err(|e| e.to_string())?;
        Ok(())
    }
    pub(crate) async fn get_gravity(&self) -> Result<Position> {
        let resp = self.c.get_gravity(Some(Empty {})).await.map_err(|e| e.to_string())?;
        Ok(resp)
    }
}
