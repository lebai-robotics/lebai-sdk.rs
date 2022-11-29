use super::Robot;
use cmod::Result;
use proto::lebai::db::*;
use proto::lebai::posture::*;
use proto::posture::{JointPose, Pose};

impl Robot {
    pub async fn kinematics_forward(&self, p: Pose) -> Result<Vec<f64>> {
        let req = PoseRequest { pose: Some(p.into()) };
        let pose = self.c.get_forward_kin(Some(req)).await.map_err(|e| e.to_string())?;
        Ok(pose.into())
    }
    pub async fn kinematics_inverse(&self, p: Pose, refer: Option<JointPose>) -> Result<JointPose> {
        let req = GetInverseKinRequest {
            pose: Some(p.into()),
            refer: refer.map(|j| j.into()),
        };
        let pose = self.c.get_inverse_kin(Some(req)).await.map_err(|e| e.to_string())?;
        Ok(pose.into())
    }

    pub async fn load_pose(&self, name: String, dir: Option<String>) -> Result<Pose> {
        let req = LoadRequest {
            name,
            dir: dir.unwrap_or_default(),
        };
        let pose = self.c.load_pose(Some(req)).await.map_err(|e| e.to_string())?;
        let pose = match pose.kind() {
            pose::Kind::Cartesian => {
                let req = PoseRequest { pose: Some(pose) };
                let pose = self.c.get_forward_kin(Some(req)).await.map_err(|e| e.to_string())?;
                Pose::Cart(pose.into())
            }
            pose::Kind::Joint => Pose::Joint(pose.joint.unwrap_or_default().into()),
        };
        Ok(pose)
    }
}
