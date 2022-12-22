use super::Robot;
use cmod::Result;
use proto::google::protobuf::Empty;
use proto::lebai::motion::*;
use proto::posture::{CartesianPose, JointPose, Pose};

impl Robot {
    pub async fn stop_move(&self) -> Result<()> {
        let req = Empty {};
        self.c.stop_move(Some(req)).await.map_err(|e| e.to_string())?;
        Ok(())
    }
    pub async fn wait_move(&self, id: Option<u32>) -> Result<()> {
        let req = MotionIndex { id: id.unwrap_or_default() };
        self.c.wait_move(Some(req)).await.map_err(|e| e.to_string())?;
        Ok(())
    }
    pub async fn get_running_motion(&self) -> Result<u32> {
        let req = Empty {};
        let id = self.c.get_running_motion(Some(req)).await.map_err(|e| e.to_string())?;
        Ok(id.id)
    }
    pub async fn get_motion_state(&self, id: u32) -> Result<String> {
        let req = MotionIndex { id };
        let res = self.c.get_motion_state(Some(req)).await.map_err(|e| e.to_string())?;
        Ok(format!("{:?}", res.state()))
    }

    pub async fn towardj(&self, p: Pose, a: f64, v: f64, t: f64, r: Option<f64>) -> Result<u32> {
        let param = MoveParam {
            acc: a,
            velocity: v,
            time: t,
            radius: r.unwrap_or_default(),
        };
        let req = MoveRequest {
            pose: Some(p.into()),
            param: Some(param),
        };
        let id = self.c.toward_joint(Some(req)).await.map_err(|e| e.to_string())?;
        Ok(id.id)
    }
    pub async fn movej(&self, p: Pose, a: f64, v: f64, t: f64, r: Option<f64>) -> Result<u32> {
        let param = MoveParam {
            acc: a,
            velocity: v,
            time: t,
            radius: r.unwrap_or_default(),
        };
        let req = MoveRequest {
            pose: Some(p.into()),
            param: Some(param),
        };
        let id = self.c.move_joint(Some(req)).await.map_err(|e| e.to_string())?;
        Ok(id.id)
    }
    pub async fn movel(&self, p: Pose, a: f64, v: f64, t: f64, r: Option<f64>) -> Result<u32> {
        let param = MoveParam {
            acc: a,
            velocity: v,
            time: t,
            radius: r.unwrap_or_default(),
        };
        let req = MoveRequest {
            pose: Some(p.into()),
            param: Some(param),
        };
        let id = self.c.move_linear(Some(req)).await.map_err(|e| e.to_string())?;
        Ok(id.id)
    }
    pub async fn movec(&self, via: Pose, p: Pose, rad: f64, a: f64, v: f64, t: f64, r: Option<f64>) -> Result<u32> {
        let param = MoveParam {
            acc: a,
            velocity: v,
            time: t,
            radius: r.unwrap_or_default(),
        };
        let req = MovecRequest {
            pose_via: Some(via.into()),
            pose: Some(p.into()),
            rad,
            param: Some(param),
        };
        let id = self.c.move_circular(Some(req)).await.map_err(|e| e.to_string())?;
        Ok(id.id)
    }

    pub async fn move_pvt(&self, p: JointPose, v: f64, t: f64) -> Result<()> {
        let joints =
            p.0.into_iter()
                .map(|x| JointMove {
                    pose: x,
                    velocity: v,
                    acc: 0.0,
                })
                .collect();
        let req = MovePvatRequest { duration: t, joints };
        self.c.move_pvat(Some(req)).await.map_err(|e| e.to_string())?;
        Ok(())
    }
    pub async fn move_pvat(&self, p: JointPose, v: f64, a: f64, t: f64) -> Result<()> {
        let joints =
            p.0.into_iter()
                .map(|x| JointMove {
                    pose: x,
                    velocity: v,
                    acc: a,
                })
                .collect();
        let req = MovePvatRequest { duration: t, joints };
        self.c.move_pvat(Some(req)).await.map_err(|e| e.to_string())?;
        Ok(())
    }

    pub async fn speedj(&self, v: JointPose) -> Result<()> {
        let param = SpeedParam {
            acc: 0.1,
            time: 0.0,
            constrained: true,
        };
        let req = SpeedJRequest {
            speed: Some(v.into()),
            param: Some(param),
        };
        self.c.speed_joint(Some(req)).await.map_err(|e| e.to_string())?;
        Ok(())
    }
    pub async fn speedl(&self, v: CartesianPose, frame: Option<CartesianPose>) -> Result<()> {
        let param = SpeedParam {
            acc: 0.1,
            time: 0.0,
            constrained: true,
        };
        let req = SpeedLRequest {
            speed: Some(v.into()),
            param: Some(param),
            frame: frame.map(|x| x.into()),
        };
        self.c.speed_linear(Some(req)).await.map_err(|e| e.to_string())?;
        Ok(())
    }
    pub async fn start_teach_mode(&self) -> Result<()> {
        let req = Empty {};
        self.c.start_teach_mode(Some(req)).await.map_err(|e| e.to_string())?;
        Ok(())
    }
    pub async fn end_teach_mode(&self) -> Result<()> {
        let req = Empty {};
        self.c.end_teach_mode(Some(req)).await.map_err(|e| e.to_string())?;
        Ok(())
    }
}
