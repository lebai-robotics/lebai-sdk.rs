use super::Robot;
use cmod::Result;
use proto::google::protobuf::Empty;
use proto::lebai::db::LoadRequest;
use proto::lebai::dynamic::*;
use proto::lebai::posture::Position;

impl Robot {
    pub(crate) async fn load_payload(&self, name: String, dir: Option<String>) -> Result<Payload> {
        let req = LoadRequest {
            name,
            dir: dir.unwrap_or_default(),
        };
        let resp = self.c.load_payload(Some(req)).await.map_err(|e| e.to_string())?;
        Ok(resp)
    }
    pub(crate) async fn set_payload(&self, mass: Option<f64>, cog: Option<Position>) -> Result<()> {
        let req = SetPayloadRequest {
            mass,
            cog,
        };
        let _ = self.c.set_payload(Some(req)).await.map_err(|e| e.to_string())?;
        Ok(())
    }
    pub(crate) async fn get_payload(&self) -> Result<Payload> {
        let resp = self.c.get_payload(Some(Empty {})).await.map_err(|e| e.to_string())?;
        Ok(resp)
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
