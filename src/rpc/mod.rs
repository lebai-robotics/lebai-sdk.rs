pub mod claw;
pub mod dynamic;
pub mod flange;
pub mod io;
pub mod kinematic;
pub mod led;
pub mod modbus;
pub mod motion;
pub mod motor;
pub mod plugin;
pub mod posture;
pub mod safety;
pub mod serial;
pub mod signal;
pub mod storage;
pub mod system;
pub mod task;

use async_lock::Mutex;
use cmod::Result;
use core::time::Duration;
use jsonrpsee_core::client::{Client, ClientT, Subscription, SubscriptionClientT};
#[cfg(target_family = "wasm")]
use jsonrpsee_wasm_client::WasmClientBuilder as WsClientBuilder;
#[cfg(not(target_family = "wasm"))]
use jsonrpsee_ws_client::{PingConfig, WsClientBuilder};
use serde_json::Value;
use std::sync::Arc;

async fn connect_ws(ip: &str, simu: bool) -> Result<Client> {
    let port: u16 = if simu { 3030 } else { 3031 };
    let mut builder = WsClientBuilder::default();
    builder = builder.request_timeout(Duration::from_secs(30 * 60));
    #[cfg(not(target_family = "wasm"))]
    let builder = builder.enable_ws_ping(PingConfig::new());
    builder.build(format!("ws://{}:{}", ip, port)).await.map_err(|e| e.to_string())
}
pub async fn connect(ip: String, simu: bool) -> Result<Robot> {
    let c = Arc::new(connect_ws(&ip, simu).await?);
    Ok(Robot { c })
}

#[derive(Clone)]
pub struct Robot {
    c: Arc<Client>,
}
impl Robot {
    pub async fn is_connected(&self) -> Result<bool> {
        Ok(self.c.is_connected())
    }
    pub async fn wait_disconnect(&self) -> Result<String> {
        let reason = self.c.on_disconnect().await;
        Ok(reason.to_string())
    }
    pub async fn call(&self, method: String, param: Option<String>) -> Result<String> {
        let mut params: Vec<Value> = Vec::new();
        if let Some(param) = param {
            let param = serde_json::from_str(&param).map_err(|e| e.to_string())?;
            params.push(param)
        }
        let res: Value = self.c.request(&method, params).await.map_err(|e| e.to_string())?;
        let res = serde_json::to_string(&res).map_err(|e| e.to_string())?;
        Ok(res)
    }
    pub async fn subscribe(&self, method: String, param: Option<String>) -> Result<RobotSubscription> {
        let subscribe_method = format!("sub_{}", method);
        let unsubscribe_method = format!("unsub_{}", method);
        let mut params: Vec<Value> = Vec::new();
        if let Some(param) = param {
            let param = serde_json::from_str(&param).map_err(|e| e.to_string())?;
            params.push(param)
        }
        let rsp = self.c.subscribe(&subscribe_method, params, &unsubscribe_method).await;
        rsp.map(|v| RobotSubscription(Arc::new(Mutex::new(v)))).map_err(|e| e.to_string())
    }
}

#[derive(Clone)]
pub struct RobotSubscription(Arc<Mutex<Subscription<Value>>>);
impl RobotSubscription {
    pub async fn next(&self) -> Result<Option<String>> {
        let res = self.0.lock().await.next().await;
        match res {
            None => Ok(None),
            Some(Ok(val)) => serde_json::to_string(&val).map(Some).map_err(|e| e.to_string()),
            Some(Err(err)) => Err(err.to_string()),
        }
    }
}
