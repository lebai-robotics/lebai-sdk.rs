use crate::lebai::posture;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct JointPose(pub Vec<f64>);
impl From<JointPose> for posture::JointPose {
    fn from(p: JointPose) -> Self {
        Self { joint: p.0 }
    }
}
impl From<posture::JointPose> for JointPose {
    fn from(p: posture::JointPose) -> Self {
        Self(p.joint)
    }
}

#[derive(Default, Serialize, Deserialize)]
pub struct CartesianPose {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub rx: Option<f64>,
    pub ry: Option<f64>,
    pub rz: Option<f64>,
}
impl From<CartesianPose> for posture::CartesianFrame {
    fn from(cart: CartesianPose) -> Self {
        let position = posture::Position {
            x: cart.x,
            y: cart.y,
            z: cart.z,
        };
        let euler = posture::EulerZyx {
            x: cart.rx.unwrap_or_default(),
            y: cart.ry.unwrap_or_default(),
            z: cart.rz.unwrap_or_default(),
        };
        let rotation = posture::Rotation {
            euler_zyx: Some(euler),
            ..Default::default()
        };
        Self {
            position_kind: posture::cartesian_frame::Kind::Custom as i32,
            position: Some(position),
            rotation_kind: posture::cartesian_frame::Kind::Custom as i32,
            rotation: Some(rotation),
        }
    }
}
impl From<CartesianPose> for posture::CartesianPose {
    fn from(cart: CartesianPose) -> Self {
        let position = posture::Position {
            x: cart.x,
            y: cart.y,
            z: cart.z,
        };
        let euler = posture::EulerZyx {
            x: cart.rx.unwrap_or_default(),
            y: cart.ry.unwrap_or_default(),
            z: cart.rz.unwrap_or_default(),
        };
        let rotation = posture::Rotation {
            euler_zyx: Some(euler),
            ..Default::default()
        };
        Self {
            position: Some(position),
            rotation: Some(rotation),
        }
    }
}
impl From<posture::CartesianPose> for CartesianPose {
    fn from(item: posture::CartesianPose) -> Self {
        let pos = item.position.unwrap_or_default();
        let rot = item.rotation.unwrap_or_default().euler_zyx.unwrap_or_default();
        CartesianPose {
            x: pos.x,
            y: pos.y,
            z: pos.z,
            rx: Some(rot.x),
            ry: Some(rot.y),
            rz: Some(rot.z),
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum Pose {
    Joint(JointPose),
    Cart(CartesianPose),
}
impl From<Pose> for posture::Pose {
    fn from(p: Pose) -> Self {
        match p {
            Pose::Joint(p) => Self {
                kind: posture::pose::Kind::Joint as i32,
                joint: Some(p.into()),
                ..Default::default()
            },
            Pose::Cart(p) => Self {
                kind: posture::pose::Kind::Cartesian as i32,
                cart: Some(p.into()),
                ..Default::default()
            },
        }
    }
}
