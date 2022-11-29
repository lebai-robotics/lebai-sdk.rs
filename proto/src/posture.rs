use crate::lebai::posture;
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
impl From<JointPose> for posture::JointPose {
    fn from(p: JointPose) -> Self {
        let mut ret = vec![p.j1, p.j2, p.j3, p.j4, p.j5];
        if let Some(j) = p.j6 {
            ret.push(j);
        }
        if let Some(j) = p.j7 {
            ret.push(j);
        }
        Self { joint: ret }
    }
}
impl From<posture::JointPose> for JointPose {
    fn from(p: posture::JointPose) -> Self {
        let j = p.joint;
        Self {
            j1: j.get(0).copied().unwrap_or_default(),
            j2: j.get(1).copied().unwrap_or_default(),
            j3: j.get(2).copied().unwrap_or_default(),
            j4: j.get(3).copied().unwrap_or_default(),
            j5: j.get(4).copied().unwrap_or_default(),
            j6: j.get(5).copied(),
            j7: j.get(6).copied(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct CartesianPose(pub Vec<f64>);
impl From<CartesianPose> for posture::CartesianPose {
    fn from(cart: CartesianPose) -> Self {
        let cart = cart.0;
        let position = posture::Position {
            x: cart.get(0).copied().unwrap_or_default(),
            y: cart.get(1).copied().unwrap_or_default(),
            z: cart.get(2).copied().unwrap_or_default(),
        };
        let euler = posture::EulerZyx {
            z: cart.get(3).copied().unwrap_or_default(),
            y: cart.get(4).copied().unwrap_or_default(),
            x: cart.get(5).copied().unwrap_or_default(),
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
        CartesianPose(vec![pos.x, pos.y, pos.z, rot.z, rot.y, rot.x])
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
