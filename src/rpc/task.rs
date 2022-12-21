use super::Robot;
use cmod::Result;
use proto::lebai::task::*;

impl Robot {
    pub(crate) async fn start_task(&self, scene: String, is_parallel: bool, loop_to: u32, dir: String, params: Vec<String>) -> Result<u32> {
        let req = StartTaskRequest {
            name: scene,
            is_parallel,
            loop_to,
            dir,
            kind: TaskKind::Lua.into(),
            params,
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
    pub(crate) async fn pause_task(&self, id: u32, time: u64, wait: bool) -> Result<()> {
        let req = PauseRequest { id, time, wait };
        let _ = self.c.pause_task(Some(req)).await.map_err(|e| e.to_string())?;
        Ok(())
    }
    pub(crate) async fn resume_task(&self, id: u32) -> Result<()> {
        let req = TaskIndex { id };
        let _ = self.c.resume_task(Some(req)).await.map_err(|e| e.to_string())?;
        Ok(())
    }
}
