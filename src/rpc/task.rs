use super::Robot;
use cmod::Result;
use proto::lebai::task::*;

impl Robot {
    pub(crate) async fn start_task(
        &self,
        scene: String,
        params: Option<Vec<String>>,
        dir: Option<String>,
        is_parallel: Option<bool>,
        loop_to: Option<u32>,
    ) -> Result<u32> {
        let req = StartTaskRequest {
            name: scene,
            is_parallel: is_parallel.unwrap_or_default(),
            loop_to: loop_to.unwrap_or(1),
            dir: dir.unwrap_or_default(),
            kind: TaskKind::Lua.into(),
            params: params.unwrap_or_default(),
        };
        let resp = self.c.start_task(Some(req)).await.map_err(|e| e.to_string())?;
        Ok(resp.id)
    }
    pub(crate) async fn get_task_state(&self, id: u32) -> Result<String> {
        let req = TaskIndex { id };
        let resp = self.c.load_task(Some(req)).await.map_err(|e| e.to_string())?;
        Ok(resp.state().as_str_name().to_string())
    }
    pub(crate) async fn cancel_task(&self, id: u32) -> Result<()> {
        let req = TaskIndex { id };
        let _ = self.c.cancel_task(Some(req)).await.map_err(|e| e.to_string())?;
        Ok(())
    }
    pub(crate) async fn pause_task(&self, id: u32) -> Result<()> {
        let req = PauseRequest { id, time: 0, wait: false };
        let _ = self.c.pause_task(Some(req)).await.map_err(|e| e.to_string())?;
        Ok(())
    }
    pub(crate) async fn resume_task(&self, id: u32) -> Result<()> {
        let req = TaskIndex { id };
        let _ = self.c.resume_task(Some(req)).await.map_err(|e| e.to_string())?;
        Ok(())
    }
}
