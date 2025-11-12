use super::Robot;
use cmod::Result;
use proto::lebai::CommandStdout;
use proto::lebai::plugin::*;

impl Robot {
    pub(crate) async fn enable_plugin(&self, name: String) -> Result<CommandStdout> {
        let req = PluginIndex { name };
        let resp = self.c.enable_plugin(Some(req)).await.map_err(|e| e.to_string())?;
        Ok(resp)
    }
    pub(crate) async fn disable_plugin(&self, name: String) -> Result<CommandStdout> {
        let req = PluginIndex { name };
        let resp = self.c.disable_plugin(Some(req)).await.map_err(|e| e.to_string())?;
        Ok(resp)
    }
    pub(crate) async fn run_plugin_cmd(&self, name: String, params: Option<Vec<String>>) -> Result<CommandStdout> {
        let req = RunPluginCmdRequest {
            name,
            params: params.unwrap_or_default(),
        };
        let resp = self.c.run_plugin_cmd(Some(req)).await.map_err(|e| e.to_string())?;
        Ok(resp)
    }
}
