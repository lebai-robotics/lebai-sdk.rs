use super::Robot;
use cmod::Result;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct JointPose {
    j1: f64,
    j2: f64,
    j3: f64,
    j4: f64,
    j5: f64,
    j6: Option<f64>,
    j7: Option<f64>,
}
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum Pose {
    Joint(JointPose),
    Cart(Vec<f64>),
}

impl Robot {
    pub async fn movej(&self, p: Pose, v: f64, a: f64, t: f64, r: Option<f64>) -> Result<u32> {
        Ok(0)
    }
    pub async fn movel(&self, p: Pose, v: f64, a: f64, t: f64, r: Option<f64>) -> Result<u32> {
        Ok(0)
    }
}
