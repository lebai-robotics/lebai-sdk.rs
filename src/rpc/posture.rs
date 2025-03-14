use super::Robot;
use cmod::Result;
use proto::lebai::db::*;
use proto::lebai::posture::{self, *};
use proto::serde::posture::{CartesianPose, JointPose, Pose};

impl Robot {
    pub async fn measure_manipulation(&self, p: JointPose) -> Result<f64> {
        let ret = self.c.measure_manipulation(Some(p.into())).await.map_err(|e| e.to_string())?;
        Ok(ret.manipulation)
    }
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
        let req = GetPoseTransRequest {
            from: Some(from.into()),
            from_to: Some(to.into()),
        };
        let pose = self.c.get_pose_trans(Some(req)).await.map_err(|e| e.to_string())?;
        Ok(pose.into())
    }
    pub async fn pose_add(&self, pose: Pose, delta: CartesianPose, frame: Option<CartesianPose>) -> Result<CartesianPose> {
        let mut delta: posture::Pose = Pose::Cart(delta).into();
        if let Some(frame) = frame {
            delta.cart_frame = Some(frame.into());
        }
        let req = GetPoseAddRequest {
            pose: Some(pose.into()),
            delta: Some(delta),
        };
        let pose = self.c.get_pose_add(Some(req)).await.map_err(|e| e.to_string())?;
        Ok(pose.into())
    }
    pub async fn pose_inverse(&self, p: Pose) -> Result<CartesianPose> {
        let req = PoseRequest { pose: Some(p.into()) };
        let pose = self.c.get_pose_inverse(Some(req)).await.map_err(|e| e.to_string())?;
        Ok(pose.into())
    }

    pub async fn save_pose(&self, name: String, pose: Option<Pose>, dir: Option<String>, refer: Option<JointPose>) -> Result<()> {
        let pose = pose.map(|x| {
            let mut x: posture::Pose = x.into();
            x.joint = refer.map(Into::into);
            x
        });
        let req = SavePoseRequest {
            name,
            dir: dir.unwrap_or_default(),
            data: pose,
        };
        self.c.save_pose(Some(req)).await.map_err(|e| e.to_string())?;
        Ok(())
    }
    pub async fn load_pose(&self, name: String, dir: Option<String>, raw_pose: Option<bool>) -> Result<Option<Pose>> {
        let raw_pose = raw_pose.unwrap_or(false);
        let req = LoadRequest {
            name,
            dir: dir.unwrap_or_default(),
        };
        let mut pose = self.c.load_pose(Some(req)).await.map_err(|e| e.to_string())?;
        let pose = match pose.kind() {
            pose::Kind::Unknown => None,
            pose::Kind::Cartesian => {
                if raw_pose {
                    let req = PoseRequest { pose: Some(pose) };
                    let pose = self.c.get_forward_kin(Some(req)).await.map_err(|e| e.to_string())?;
                    Some(Pose::Cart(pose.into()))
                } else {
                    let refer = pose.joint.take();
                    let req = GetInverseKinRequest { pose: Some(pose), refer };
                    let pose = self.c.get_inverse_kin(Some(req)).await.map_err(|e| e.to_string())?;
                    Some(Pose::Joint(pose.into()))
                }
            }
            pose::Kind::Joint => Some(Pose::Joint(pose.joint.unwrap_or_default().into())),
        };
        Ok(pose)
    }
    pub async fn load_frame(&self, name: String, dir: Option<String>) -> Result<CartesianPose> {
        let frame_index = LoadRequest {
            name,
            dir: dir.unwrap_or_default(),
        };
        let req = PoseRequest {
            pose: Some(posture::Pose {
                kind: posture::pose::Kind::Cartesian as i32,
                cart_frame_index: Some(frame_index),
                ..Default::default()
            }),
        };
        let pose = self.c.get_forward_kin(Some(req)).await.map_err(|e| e.to_string())?;
        let rot = pose.rotation.unwrap_or_default().euler_zyx.unwrap_or_default();
        Ok(CartesianPose {
            rx: rot.x,
            ry: rot.y,
            rz: rot.z,
            ..Default::default()
        })
    }
}
