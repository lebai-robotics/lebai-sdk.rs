use crate::lebai::kinematic;
use crate::serde::posture::{CartesianPose, JointPose};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct KinData {
    pub actual_joint_pose: JointPose,
    pub actual_joint_speed: JointPose,
    pub actual_joint_acc: JointPose,
    pub actual_joint_torque: JointPose,
    pub target_joint_pose: JointPose,
    pub target_joint_speed: JointPose,
    pub target_joint_acc: JointPose,
    pub target_joint_torque: JointPose,

    pub actual_tcp_pose: CartesianPose,
    pub target_tcp_pose: CartesianPose,

    pub actual_flange_pose: CartesianPose,
}

impl From<kinematic::KinData> for KinData {
    fn from(p: kinematic::KinData) -> Self {
        Self {
            actual_joint_pose: JointPose(p.actual_joint_pose),
            actual_joint_speed: JointPose(p.actual_joint_speed),
            actual_joint_acc: JointPose(p.actual_joint_acc),
            actual_joint_torque: JointPose(p.actual_joint_torque),
            target_joint_pose: JointPose(p.target_joint_pose),
            target_joint_speed: JointPose(p.target_joint_speed),
            target_joint_acc: JointPose(p.target_joint_acc),
            target_joint_torque: JointPose(p.target_joint_torque),

            actual_tcp_pose: p.actual_tcp_pose.unwrap_or_default().into(),
            target_tcp_pose: p.target_tcp_pose.unwrap_or_default().into(),

            actual_flange_pose: p.actual_flange_pose.unwrap_or_default().into(),
        }
    }
}
