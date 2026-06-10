pub mod auto;
pub mod claw;
pub mod dynamic;
pub mod flange;
pub mod io;
pub mod kinematic;
pub mod led;
pub mod modbus;
pub mod motion;
pub mod motor;
pub mod network;
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
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::sync::Arc;

#[derive(Deserialize, Serialize)]
#[serde(untagged)]
pub enum RobotPort {
    Simu(bool),
    Port(u16),
}
impl From<u16> for RobotPort {
    fn from(value: u16) -> Self {
        Self::Port(value)
    }
}
#[derive(Deserialize, Serialize)]
pub struct ConnectOption {
    pub request_timeout: u32,
}

async fn connect_ws(url: &str, option: ConnectOption) -> Result<Client> {
    let mut builder = WsClientBuilder::default();
    builder = builder.request_timeout(Duration::from_secs(option.request_timeout as u64));
    #[cfg(not(target_family = "wasm"))]
    let builder = builder.enable_ws_ping(PingConfig::new());
    builder.build(url).await.map_err(|e| e.to_string())
}

/// IP样式1: 192.168.2.101
/// IP样式2: user:passwd@22dadd5c5fd0.nodes.lebai.ltd
pub async fn connect(ip: String, port: Option<RobotPort>, option: Option<ConnectOption>) -> Result<Robot> {
    if let Some((scheme, _)) = ip.split_once("://") {
        return Err(format!("ip format error: {}://", scheme));
    }
    if let Some(port) = ip.split(':').next_back()
        && port.chars().all(|x| x.is_ascii_digit())
    {
        return Err(format!("ip format error: :{}", port));
    }

    let is_frpc = ip.contains("lebai.ltd");
    let url = match port.unwrap_or(RobotPort::Simu(false)) {
        RobotPort::Simu(simu) => match (is_frpc, simu) {
            (true, true) => format!("ws://{}/sim/rpc/ws:80", ip),
            (true, false) => format!("ws://{}/rpc/ws:80", ip),
            (false, true) => format!("ws://{}:3030", ip),
            (false, false) => format!("ws://{}:3031", ip),
        },
        RobotPort::Port(port) => {
            if is_frpc {
                format!("ws://{}/sim/rpc/ws:{}", ip, port)
            } else {
                format!("ws://{}:{}", ip, port)
            }
        }
    };
    let option = option.unwrap_or(ConnectOption { request_timeout: 30 * 60 });
    let c = Arc::new(connect_ws(&url, option).await?);
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
