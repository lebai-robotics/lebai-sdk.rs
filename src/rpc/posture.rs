use super::Robot;
use cmod::Result;
use proto::lebai::db::*;
use proto::lebai::posture::*;
use proto::posture::{CartesianPose, JointPose, Pose};

impl Robot {
    pub async fn kinematics_forward(&self, p: Pose) -> Result<CartesianPose> {
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
    pub async fn pose_trans(&self, from: Pose, to: Pose) -> Result<CartesianPose> {
        let req = GetPoseMultiplyRequest {
            base: Some(from.into()),
            target: Some(to.into()),
        };
        let pose = self.c.get_pose_multiply(Some(req)).await.map_err(|e| e.to_string())?;
        Ok(pose.into())
    }
    pub async fn pose_inverse(&self, p: Pose) -> Result<CartesianPose> {
        let req = PoseRequest { pose: Some(p.into()) };
        let pose = self.c.get_pose_inverse(Some(req)).await.map_err(|e| e.to_string())?;
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
    pub async fn load_frame(&self, name: String, dir: Option<String>) -> Result<CartesianPose> {
        let req = LoadRequest {
            name,
            dir: dir.unwrap_or_default(),
        };
        let pose = self.c.load_frame(Some(req)).await.map_err(|e| e.to_string())?;
        let pose = match pose.rotation_kind() {
            cartesian_frame::Kind::Base => {
                vec![0.0; 6]
            }
            cartesian_frame::Kind::Flange => todo!(),
            cartesian_frame::Kind::LastFlange => todo!(),
            cartesian_frame::Kind::Tcp => todo!(),
            cartesian_frame::Kind::LastTcp => todo!(),
            cartesian_frame::Kind::Custom => {
                let rot = pose.rotation.unwrap_or_default().euler_zyx.unwrap_or_default();
                vec![0.0, 0.0, 0.0, rot.z, rot.y, rot.x]
            }
        };
        Ok(CartesianPose(pose))
    }
}
