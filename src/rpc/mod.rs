pub mod motion;
pub mod posture;

use crate::RT;
use cmod::Result;
use jsonrpsee_core::client::{ClientT, Subscription, SubscriptionClientT};
use jsonrpsee_ws_client::{WsClient, WsClientBuilder};
use serde_json::Value;
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn connect(ip: String, simu: bool) -> Result<Robot> {
    let _rt = RT.enter();
    let port: u16 = if simu { 3030 } else { 3031 };
    let client = WsClientBuilder::default()
        .build(format!("ws://{}:{}", ip, port))
        .await
        .map_err(|e| e.to_string())?;
    Ok(Robot { c: Arc::new(client) })
}

#[derive(Clone)]
pub struct Robot {
    c: Arc<WsClient>,
}
impl Robot {
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
