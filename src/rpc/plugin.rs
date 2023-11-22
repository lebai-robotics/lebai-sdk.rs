use super::Robot;
use cmod::Result;
use proto::lebai::plugin::*;
use proto::lebai::CommandStdout;

impl Robot {
    pub(crate) async fn run_plugin_cmd(
        &self,
        name: String,
        params: Option<Vec<String>>,
    ) -> Result<CommandStdout> {
        let req = RunPluginCmdRequest {
            name,
            params: params.unwrap_or_default(),
        };
        let resp = self.c.run_plugin_cmd(Some(req)).await.map_err(|e| e.to_string())?;
        Ok(resp)
    }
}
