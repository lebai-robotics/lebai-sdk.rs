mod common;
#[cfg(feature = "mdns")]
mod mdns;
mod rpc;
#[cfg(not(target_family = "wasm"))]
mod runtime;

#[cmod::cmod]
pub mod lebai_sdk {
    use super::*;
    use cmod::Result;
    use proto::io::IoDevice;
    use proto::posture::{CartesianPose, JointPose, Pose};

    #[cmod::function]
    pub fn version() -> Result<String> {
        Ok(common::VERSION.into())
    }

    #[cmod::function]
    #[cmod::tags(ret)]
    pub async fn discover_devices(time: u32) -> Result<Vec<proto::lebai::multi_devices::DeviceInfo>> {
        #[cfg(not(feature = "mdns"))]
        {
            drop(time);
            return Err("not support".into());
        }
        #[cfg(feature = "mdns")]
        mdns::discover_devices(time).await
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
        #[cmod::tags(args(p), ret)]
        pub async fn kinematics_forward(&self, p: Pose) -> Result<CartesianPose> {
            self.0.kinematics_forward(p).await
        }
        #[classmethod]
        #[cmod::tags(args(p, refer), ret)]
        pub async fn kinematics_inverse(&self, p: Pose, refer: Option<JointPose>) -> Result<JointPose> {
            self.0.kinematics_inverse(p, refer).await
        }
        #[classmethod]
        #[cmod::tags(args(from, to), ret)]
        pub async fn pose_trans(&self, from: Pose, to: Pose) -> Result<CartesianPose> {
            self.0.pose_trans(from, to).await
        }
        #[classmethod]
        #[cmod::tags(args(p), ret)]
        pub async fn pose_inverse(&self, p: Pose) -> Result<CartesianPose> {
            self.0.pose_inverse(p).await
        }
        #[classmethod]
        #[cmod::tags(ret)]
        pub async fn load_pose(&self, name: String, dir: Option<String>) -> Result<Pose> {
            self.0.load_pose(name, dir).await
        }
        #[classmethod]
        #[cmod::tags(ret)]
        pub async fn load_frame(&self, name: String, dir: Option<String>) -> Result<CartesianPose> {
            self.0.load_frame(name, dir).await
        }

        #[classmethod]
        pub async fn stop_move(&self) -> Result<()> {
            self.0.stop_move().await
        }
        #[classmethod]
        pub async fn wait_move(&self, id: u32) -> Result<()> {
            self.0.wait_move(id).await
        }
        #[classmethod]
        pub async fn get_running_motion(&self) -> Result<u32> {
            self.0.get_running_motion().await
        }
        #[classmethod]
        pub async fn get_motion_state(&self, id: u32) -> Result<String> {
            self.0.get_motion_state(id).await
        }
        #[classmethod]
        #[cmod::tags(args(p))]
        pub async fn towardj(&self, p: Pose, a: f64, v: f64, t: f64, r: Option<f64>) -> Result<u32> {
            self.0.towardj(p, a, v, t, r).await
        }
        #[classmethod]
        #[cmod::tags(args(p))]
        pub async fn movej(&self, p: Pose, a: f64, v: f64, t: f64, r: Option<f64>) -> Result<u32> {
            self.0.movej(p, a, v, t, r).await
        }
        #[classmethod]
        #[cmod::tags(args(p))]
        pub async fn movel(&self, p: Pose, a: f64, v: f64, t: f64, r: Option<f64>) -> Result<u32> {
            self.0.movel(p, a, v, t, r).await
        }
        #[classmethod]
        #[cmod::tags(args(via, p))]
        pub async fn movec(&self, via: Pose, p: Pose, rad: f64, a: f64, v: f64, t: f64, r: Option<f64>) -> Result<u32> {
            self.0.movec(via, p, rad, a, v, t, r).await
        }
        #[classmethod]
        #[cmod::tags(args(p))]
        pub async fn move_pvt(&self, p: JointPose, v: f64, t: f64) -> Result<()> {
            self.0.move_pvt(p, v, t).await
        }
        #[classmethod]
        #[cmod::tags(args(p))]
        pub async fn move_pvat(&self, p: JointPose, v: f64, a: f64, t: f64) -> Result<()> {
            self.0.move_pvat(p, v, a, t).await
        }
        #[classmethod]
        #[cmod::tags(args(v))]
        pub async fn speedj(&self, v: JointPose) -> Result<()> {
            self.0.speedj(v).await
        }
        #[classmethod]
        #[cmod::tags(args(v, frame))]
        pub async fn speedl(&self, v: CartesianPose, frame: Option<CartesianPose>) -> Result<()> {
            self.0.speedl(v, frame).await
        }
        #[classmethod]
        pub async fn start_teach_mode(&self) -> Result<()> {
            self.0.start_teach_mode().await
        }
        #[classmethod]
        pub async fn end_teach_mode(&self) -> Result<()> {
            self.0.end_teach_mode().await
        }

        //IO
        #[classmethod]
        #[cmod::tags(args(device))]
        pub async fn set_do(&self, device: IoDevice, pin: u32, value: u32) -> Result<()> {
            self.0.set_do(device, pin, value).await
        }
        #[classmethod]
        #[cmod::tags(args(device))]
        pub async fn get_do(&self, device: IoDevice, pin: u32) -> Result<u32> {
            self.0.get_do(device, pin).await
        }
        #[classmethod]
        #[cmod::tags(args(device))]
        pub async fn get_dos(&self, device: IoDevice, pin: u32, num: u32) -> Result<Vec<u32>> {
            self.0.get_dos(device, pin, num).await
        }
        #[classmethod]
        #[cmod::tags(args(device))]
        pub async fn get_di(&self, device: IoDevice, pin: u32) -> Result<u32> {
            self.0.get_di(device, pin).await
        }
        #[classmethod]
        #[cmod::tags(args(device))]
        pub async fn get_dis(&self, device: IoDevice, pin: u32, num: u32) -> Result<Vec<u32>> {
            self.0.get_dis(device, pin, num).await
        }
        #[classmethod]
        #[cmod::tags(args(device))]
        pub async fn set_ao(&self, device: IoDevice, pin: u32, value: u32) -> Result<()> {
            self.0.set_ao(device, pin, value).await
        }
        #[classmethod]
        #[cmod::tags(args(device))]
        pub async fn get_ao(&self, device: IoDevice, pin: u32) -> Result<f64> {
            self.0.get_ao(device, pin).await
        }
        #[classmethod]
        #[cmod::tags(args(device))]
        pub async fn get_aos(&self, device: IoDevice, pin: u32, num: u32) -> Result<Vec<f64>> {
            self.0.get_aos(device, pin, num).await
        }
        #[classmethod]
        #[cmod::tags(args(device))]
        pub async fn get_ai(&self, device: IoDevice, pin: u32) -> Result<f64> {
            self.0.get_ai(device, pin).await
        }
        #[classmethod]
        #[cmod::tags(args(device))]
        pub async fn get_ais(&self, device: IoDevice, pin: u32, num: u32) -> Result<Vec<f64>> {
            self.0.get_ais(device, pin, num).await
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
