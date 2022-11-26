mod common;
mod rpc;

use once_cell::sync::Lazy;
use tokio::runtime::{Builder, Runtime};

static RT: Lazy<Runtime> = Lazy::new(|| {
    Builder::new_multi_thread()
        .worker_threads(1)
        .thread_name("lua-sdk")
        .thread_stack_size(3 * 1024 * 1024)
        .enable_all()
        .build()
        .unwrap()
});

#[cmod::cmod]
pub mod lebai_sdk {
    use super::*;
    use cmod::Result;

    #[cmod::function]
    pub fn version() -> Result<String> {
        Ok(common::VERSION.into())
    }
    #[cmod::function]
    pub async fn connect(ip: String, simu: bool) -> Result<Robot> {
        let robot = rpc::connect(ip, simu).await?;
        Ok(Robot(robot))
    }

    #[cmod::class]
    #[derive(Clone)]
    pub struct Robot(rpc::Robot);
    #[cmod::methods]
    impl Robot {
        #[classmethod]
        pub async fn call(&self, method: String, param: Option<String>) -> Result<String> {
            self.0.call(method, param).await
        }
        #[classmethod]
        pub async fn subscribe(&self, method: String, param: Option<String>) -> Result<RobotSubscription> {
            let subscription = self.0.subscribe(method, param).await?;
            Ok(RobotSubscription(subscription))
        }

        #[classmethod]
        #[cmod::tags(args(p))]
        pub async fn movej(&self, p: proto::posture::Pose, a: f64, v: f64, t: f64, r: Option<f64>) -> Result<u32> {
            self.0.movej(p, v, a, t, r).await
        }
        #[classmethod]
        #[cmod::tags(args(p))]
        pub async fn movel(&self, p: proto::posture::Pose, a: f64, v: f64, t: f64, r: Option<f64>) -> Result<u32> {
            self.0.movel(p, v, a, t, r).await
        }
    }
    #[cmod::class]
    #[derive(Clone)]
    pub struct RobotSubscription(rpc::RobotSubscription);
    #[cmod::methods]
    impl RobotSubscription {
        #[classmethod]
        pub async fn next(&self) -> Result<Option<String>> {
            self.0.next().await
        }
    }
}
