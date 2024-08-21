use super::Robot;
use cmod::Result;
use proto::google::protobuf::Empty;
use proto::lebai::motor::*;

impl Robot {
    pub(crate) async fn find_zero(&self) -> Result<()> {
        let _ = self.c.find_zero(Some(Empty {})).await.map_err(|e| e.to_string())?;
        Ok(())
    }
}
