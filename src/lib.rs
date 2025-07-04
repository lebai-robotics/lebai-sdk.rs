mod common;
#[cfg(feature = "mdns")]
mod mdns;
mod rpc;
mod runtime;

#[cmod::cmod]
mod lebai_sdk {
    use super::*;
    use cmod::Result;
    use proto::lebai::claw::Claw;
    use proto::lebai::dynamic::Payload;
    use proto::lebai::io::DigitalMode;
    use proto::lebai::posture::Position;
    use proto::lebai::system::{EstopReason, PhyData, RobotState};
    use proto::serde::kinematic::KinData;
    use proto::serde::led::LedStyle;
    use proto::serde::posture::{CartesianPose, JointPose, Pose};
    use runtime::CompatExt as _;

    #[cmod::function]
    pub fn version() -> Result<String> {
        Ok(common::VERSION.into())
    }
    #[cfg(not(target_family = "wasm"))]
    #[cmod::function]
    pub fn init() -> Result<()> {
        #[cfg(feature = "ffi_py")]
        let _ = cmod::ffi::py::init_runtime(&*runtime::RT);

        Ok(())
    }
    #[cmod::function]
    #[cmod::tags(ret)]
    pub fn timestamp() -> Result<u64> {
        common::timestamp()
    }
    #[cmod::function]
    #[cmod::tags(args(ms))]
    pub async fn sleep_ms(ms: u64) -> Result<()> {
        common::sleep_ms(ms).compat().await
    }

    #[cmod::function]
    #[cmod::tags(ret)]
    pub async fn discover_devices(time: f64) -> Result<Vec<proto::lebai::multi_devices::DeviceInfo>> {
        #[cfg(not(feature = "mdns"))]
        {
            let _time = time;
            Err("not support".into())
        }
        #[cfg(feature = "mdns")]
        mdns::discover_devices(time).compat().await
    }

    #[cmod::function]
    pub async fn connect(ip: String, simu: bool) -> Result<Robot> {
        let robot = rpc::connect(ip, simu).compat().await?;
        Ok(Robot(robot))
    }

    #[cmod::class]
    #[derive(Clone)]
    pub struct Robot(rpc::Robot);
    #[cmod::methods]
    impl Robot {
        #[classmethod]
        pub async fn is_connected(&self) -> Result<bool> {
            self.0.is_connected().await
        }
        #[classmethod]
        pub async fn wait_disconnect(&self) -> Result<String> {
            self.0.wait_disconnect().await
        }
        #[classmethod]
        pub async fn call(&self, method: String, param: Option<String>) -> Result<String> {
            self.0.call(method, param).await
        }
        #[classmethod]
        pub async fn subscribe(&self, method: String, param: Option<String>) -> Result<RobotSubscription> {
            let subscription = self.0.subscribe(method, param).await?;
            Ok(RobotSubscription(subscription))
        }

        // Posture
        #[classmethod]
        #[cmod::tags(args(p))]
        pub async fn measure_manipulation(&self, p: JointPose) -> Result<f64> {
            self.0.measure_manipulation(p).await
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
        #[cmod::tags(args(pose, frame, delta), ret)]
        pub async fn pose_add(&self, pose: Pose, delta: CartesianPose, frame: Option<CartesianPose>) -> Result<CartesianPose> {
            self.0.pose_add(pose, delta, frame).await
        }
        #[classmethod]
        #[cmod::tags(args(p), ret)]
        pub async fn pose_inverse(&self, p: Pose) -> Result<CartesianPose> {
            self.0.pose_inverse(p).await
        }
        #[classmethod]
        #[cmod::tags(args(pose, refer))]
        pub async fn save_pose(&self, name: String, pose: Option<Pose>, dir: Option<String>, refer: Option<JointPose>) -> Result<()> {
            self.0.save_pose(name, pose, dir, refer).await
        }
        #[classmethod]
        #[cmod::tags(ret)]
        pub async fn load_pose(&self, name: String, dir: Option<String>, raw_pose: Option<bool>) -> Result<Option<Pose>> {
            self.0.load_pose(name, dir, raw_pose).await
        }
        #[classmethod]
        #[cmod::tags(ret)]
        pub async fn load_frame(&self, name: String, dir: Option<String>) -> Result<CartesianPose> {
            self.0.load_frame(name, dir).await
        }

        // Motion
        #[classmethod]
        pub async fn pause_move(&self) -> Result<()> {
            self.0.pause_move().await
        }
        #[classmethod]
        pub async fn resume_move(&self) -> Result<()> {
            self.0.resume_move().await
        }
        #[classmethod]
        pub async fn stop_move(&self) -> Result<()> {
            self.0.stop_move().await
        }
        #[classmethod]
        pub async fn wait_move(&self, id: Option<u32>) -> Result<()> {
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
        pub async fn towardj(&self, p: Pose, a: f64, v: f64, t: Option<f64>, r: Option<f64>) -> Result<u32> {
            self.0.towardj(p, a, v, t, r).await
        }
        #[classmethod]
        #[cmod::tags(args(p))]
        pub async fn movej(&self, p: Pose, a: f64, v: f64, t: Option<f64>, r: Option<f64>) -> Result<u32> {
            self.0.movej(p, a, v, t, r).await
        }
        #[classmethod]
        #[cmod::tags(args(p))]
        pub async fn movel(&self, p: Pose, a: f64, v: f64, t: Option<f64>, r: Option<f64>) -> Result<u32> {
            self.0.movel(p, a, v, t, r).await
        }
        #[classmethod]
        #[cmod::tags(args(via, p))]
        pub async fn movec(&self, via: Pose, p: Pose, rad: f64, a: f64, v: f64, t: Option<f64>, r: Option<f64>) -> Result<u32> {
            self.0.movec(via, p, rad, a, v, t, r).await
        }
        #[classmethod]
        #[cmod::tags(args(p, v, a))]
        pub async fn move_pvat(&self, p: JointPose, v: Vec<f64>, a: Vec<f64>, t: f64) -> Result<()> {
            self.0.move_pvat(p, v, a, t).await
        }
        #[classmethod]
        #[cmod::tags(args(p, v))]
        pub async fn move_pvt(&self, p: JointPose, v: Vec<f64>, t: f64) -> Result<()> {
            self.0.move_pvt(p, v, t).await
        }
        #[classmethod]
        #[cmod::tags(args(p))]
        pub async fn move_pt(&self, p: JointPose, t: f64) -> Result<()> {
            self.0.move_pt(p, t).await
        }
        #[classmethod]
        pub async fn move_trajectory(&self, name: String, dir: Option<String>) -> Result<u32> {
            self.0.move_trajectory(name, dir).await
        }
        #[classmethod]
        #[cmod::tags(args(v))]
        pub async fn speedj(&self, a: f64, v: JointPose, t: Option<f64>) -> Result<u32> {
            self.0.speedj(a, v, t).await
        }
        #[classmethod]
        #[cmod::tags(args(v, frame))]
        pub async fn speedl(&self, a: f64, v: CartesianPose, t: Option<f64>, frame: Option<CartesianPose>) -> Result<u32> {
            self.0.speedl(a, v, t, frame).await
        }
        #[classmethod]
        pub async fn teach_mode(&self) -> Result<()> {
            self.0.start_teach_mode().await
        }
        #[classmethod]
        pub async fn end_teach_mode(&self) -> Result<()> {
            self.0.end_teach_mode().await
        }

        //IO
        #[classmethod]
        #[cmod::tags(args(device, mode))]
        pub async fn set_dio_mode(&self, device: String, pin: u32, mode: DigitalMode) -> Result<()> {
            self.0.set_dio_mode(device, pin, mode).await
        }
        #[classmethod]
        #[cmod::tags(args(device), ret)]
        pub async fn get_dio_mode(&self, device: String, pin: u32) -> Result<DigitalMode> {
            self.0.get_dio_mode(device, pin).await
        }
        #[classmethod]
        #[cmod::tags(args(device))]
        pub async fn set_do(&self, device: String, pin: u32, value: u32) -> Result<()> {
            self.0.set_do(device, pin, value).await
        }
        #[classmethod]
        #[cmod::tags(args(device))]
        pub async fn get_do(&self, device: String, pin: u32) -> Result<u32> {
            self.0.get_do(device, pin).await
        }
        #[classmethod]
        #[cmod::tags(args(device), ret)]
        pub async fn get_dos(&self, device: String, pin: u32, num: u32) -> Result<Vec<u32>> {
            self.0.get_dos(device, pin, num).await
        }
        #[classmethod]
        #[cmod::tags(args(device))]
        pub async fn get_di(&self, device: String, pin: u32) -> Result<u32> {
            self.0.get_di(device, pin).await
        }
        #[classmethod]
        #[cmod::tags(args(device), ret)]
        pub async fn get_dis(&self, device: String, pin: u32, num: u32) -> Result<Vec<u32>> {
            self.0.get_dis(device, pin, num).await
        }
        #[classmethod]
        #[cmod::tags(args(device))]
        pub async fn set_ao(&self, device: String, pin: u32, value: f64) -> Result<()> {
            self.0.set_ao(device, pin, value).await
        }
        #[classmethod]
        #[cmod::tags(args(device))]
        pub async fn get_ao(&self, device: String, pin: u32) -> Result<f64> {
            self.0.get_ao(device, pin).await
        }
        #[classmethod]
        #[cmod::tags(args(device), ret)]
        pub async fn get_aos(&self, device: String, pin: u32, num: u32) -> Result<Vec<f64>> {
            self.0.get_aos(device, pin, num).await
        }
        #[classmethod]
        #[cmod::tags(args(device))]
        pub async fn get_ai(&self, device: String, pin: u32) -> Result<f64> {
            self.0.get_ai(device, pin).await
        }
        #[classmethod]
        #[cmod::tags(args(device), ret)]
        pub async fn get_ais(&self, device: String, pin: u32, num: u32) -> Result<Vec<f64>> {
            self.0.get_ais(device, pin, num).await
        }

        //SIGNAL
        #[classmethod]
        pub async fn set_signal(&self, index: u32, value: i32) -> Result<()> {
            self.0.set_signal(index, value).await
        }
        #[classmethod]
        #[cmod::tags(args(values))]
        pub async fn set_signals(&self, index: u32, values: Vec<i32>) -> Result<()> {
            self.0.set_signals(index, values).await
        }
        #[classmethod]
        pub async fn get_signal(&self, index: u32) -> Result<i32> {
            self.0.get_signal(index).await
        }
        #[classmethod]
        #[cmod::tags(ret)]
        pub async fn get_signals(&self, index: u32, len: u32) -> Result<Vec<i32>> {
            self.0.get_signals(index, len).await
        }
        #[classmethod]
        pub async fn add_signal(&self, index: u32, value: i32) -> Result<()> {
            self.0.add_signal(index, value).await
        }

        // Storage
        #[classmethod]
        pub async fn set_item(&self, key: String, value: String) -> Result<()> {
            self.0.set_item(key, value).await
        }
        #[classmethod]
        #[cmod::tags(ret)]
        pub async fn get_item(&self, key: String) -> Result<proto::lebai::storage::Item> {
            self.0.get_item(key).await
        }
        #[classmethod]
        #[cmod::tags(ret)]
        pub async fn get_items(&self, prefix: String) -> Result<Vec<proto::lebai::storage::Item>> {
            self.0.get_items(prefix).await
        }

        //Plugin
        #[classmethod]
        #[cmod::tags(args(params), ret)]
        pub async fn run_plugin_cmd(&self, name: String, params: Option<Vec<String>>) -> Result<proto::lebai::CommandStdout> {
            self.0.run_plugin_cmd(name, params).await
        }
        //TASK
        #[classmethod]
        #[cmod::tags(args(params))]
        pub async fn start_task(
            &self,
            scene: String,
            params: Option<Vec<String>>,
            dir: Option<String>,
            is_parallel: Option<bool>,
            loop_to: Option<u32>,
        ) -> Result<u32> {
            self.0.start_task(scene, params, dir, is_parallel, loop_to).await
        }
        #[classmethod]
        #[cmod::tags(ret)]
        pub async fn get_task_list(&self) -> Result<Vec<u32>> {
            self.0.get_task_list().await
        }
        #[classmethod]
        pub async fn get_main_task_id(&self) -> Result<Option<u32>> {
            self.0.get_main_task_id().await
        }
        #[classmethod]
        pub async fn wait_task(&self, id: Option<u32>) -> Result<String> {
            self.0.wait_task(id).await
        }
        #[classmethod]
        pub async fn get_task_state(&self, id: Option<u32>) -> Result<String> {
            self.0.get_task_state(id).await
        }
        #[classmethod]
        pub async fn cancel_task(&self, id: Option<u32>) -> Result<()> {
            self.0.cancel_task(id).await
        }
        #[classmethod]
        pub async fn pause_task(&self, id: Option<u32>) -> Result<()> {
            self.0.pause_task(id).await
        }
        #[classmethod]
        pub async fn resume_task(&self, id: Option<u32>) -> Result<()> {
            self.0.resume_task(id).await
        }

        // Serial
        #[classmethod]
        pub async fn set_serial_timeout(&self, device: String, timeout: u32) -> Result<()> {
            self.0.set_serial_timeout(device, timeout).await
        }
        #[classmethod]
        pub async fn set_serial_baud_rate(&self, device: String, baud_rate: u32) -> Result<()> {
            self.0.set_serial_baud_rate(device, baud_rate).await
        }
        #[classmethod]
        #[cmod::tags(args(parity))]
        pub async fn set_serial_parity(&self, device: String, parity: proto::lebai::serial::Parity) -> Result<()> {
            self.0.set_serial_parity(device, parity).await
        }
        #[classmethod]
        #[cmod::tags(args(data))]
        pub async fn write_serial(&self, device: String, data: Vec<u8>) -> Result<()> {
            self.0.write_serial(device, data).await
        }
        #[classmethod]
        #[cmod::tags(ret)]
        pub async fn read_serial(&self, device: String, len: u32) -> Result<Vec<u8>> {
            self.0.read_serial(device, len).await
        }
        #[classmethod]
        pub async fn clear_serial(&self, device: String) -> Result<()> {
            self.0.clear_serial(device).await
        }

        //MODBUS
        #[classmethod]
        pub async fn disconnect_modbus(&self, device: String) -> Result<()> {
            self.0.disconnect_modbus(device).await
        }
        #[classmethod]
        pub async fn set_modbus_timeout(&self, device: String, timeout: u32) -> Result<()> {
            self.0.set_modbus_timeout(device, timeout).await
        }
        #[classmethod]
        pub async fn set_modbus_retry(&self, device: String, retry: u32) -> Result<()> {
            self.0.set_modbus_retry(device, retry).await
        }
        #[classmethod]
        pub async fn write_single_coil(&self, device: String, pin: String, value: bool) -> Result<()> {
            self.0.write_single_coil(device, pin, value).await
        }
        #[classmethod]
        #[cmod::tags(args(values))]
        pub async fn write_multiple_coils(&self, device: String, pin: String, values: Vec<bool>) -> Result<()> {
            self.0.write_multiple_coils(device, pin, values).await
        }
        #[classmethod]
        #[cmod::tags(ret)]
        pub async fn read_coils(&self, device: String, pin: String, count: u32) -> Result<Vec<bool>> {
            self.0.read_coils(device, pin, count).await
        }
        #[classmethod]
        #[cmod::tags(ret)]
        pub async fn read_discrete_inputs(&self, device: String, pin: String, count: u32) -> Result<Vec<bool>> {
            self.0.read_discrete_inputs(device, pin, count).await
        }
        #[classmethod]
        pub async fn write_single_register(&self, device: String, pin: String, value: u32) -> Result<()> {
            self.0.write_single_register(device, pin, value).await
        }
        #[classmethod]
        #[cmod::tags(args(values))]
        pub async fn write_multiple_registers(&self, device: String, pin: String, values: Vec<u32>) -> Result<()> {
            self.0.write_multiple_registers(device, pin, values).await
        }
        #[classmethod]
        #[cmod::tags(ret)]
        pub async fn read_holding_registers(&self, device: String, pin: String, count: u32) -> Result<Vec<u32>> {
            self.0.read_holding_registers(device, pin, count).await
        }
        #[classmethod]
        #[cmod::tags(ret)]
        pub async fn read_input_registers(&self, device: String, pin: String, count: u32) -> Result<Vec<u32>> {
            self.0.read_input_registers(device, pin, count).await
        }

        //CLAW
        #[classmethod]
        pub async fn init_claw(&self, force: Option<bool>) -> Result<()> {
            self.0.init_claw(force).await
        }
        #[classmethod]
        pub async fn set_claw(&self, force: Option<f64>, amplitude: Option<f64>) -> Result<()> {
            self.0.set_claw(force, amplitude).await
        }
        #[classmethod]
        #[cmod::tags(ret)]
        pub async fn get_claw(&self) -> Result<Claw> {
            self.0.get_claw().await
        }

        //Flange
        #[classmethod]
        pub async fn set_flange_baud_rate(&self, baud_rate: u32) -> Result<()> {
            self.0.set_flange_baud_rate(baud_rate).await
        }

        //LED
        #[classmethod]
        #[cmod::tags(ret)]
        pub async fn load_led_style(&self, name: String, dir: Option<String>) -> Result<LedStyle> {
            self.0.load_led_style(name, dir).await
        }
        #[classmethod]
        #[cmod::tags(args(style))]
        pub async fn set_led_style(&self, style: LedStyle) -> Result<()> {
            self.0.set_led_style(style).await
        }
        #[classmethod]
        pub async fn set_led(&self, mode: i32, speed: i32, colors: Vec<i32>) -> Result<()> {
            self.0.set_led(mode, speed, colors).await
        }
        #[classmethod]
        pub async fn set_voice(&self, voice: i32, volume: i32) -> Result<()> {
            self.0.set_voice(voice, volume).await
        }
        #[classmethod]
        pub async fn set_fan(&self, mode: i32) -> Result<()> {
            self.0.set_fan(mode).await
        }

        //SYSTEM
        #[classmethod]
        pub async fn start_sys(&self) -> Result<()> {
            self.0.start_sys().await
        }
        #[classmethod]
        pub async fn stop_sys(&self) -> Result<()> {
            self.0.stop_sys().await
        }
        #[classmethod]
        pub async fn powerdown(&self) -> Result<()> {
            self.0.powerdown().await
        }
        #[classmethod]
        pub async fn reboot(&self) -> Result<()> {
            self.0.reboot().await
        }
        #[classmethod]
        pub async fn stop(&self) -> Result<()> {
            self.0.stop().await
        }
        #[classmethod]
        pub async fn estop(&self) -> Result<()> {
            self.0.estop().await
        }

        //Safety
        #[classmethod]
        pub async fn set_collision_detector_sensitivity(&self, sensitivity: u32) -> Result<()> {
            self.0.set_collision_detector_sensitivity(sensitivity).await
        }
        #[classmethod]
        pub async fn disable_collision_detector(&self) -> Result<()> {
            self.0.disable_collision_detector().await
        }
        #[classmethod]
        pub async fn enable_collision_detector(&self) -> Result<()> {
            self.0.enable_collision_detector().await
        }
        #[classmethod]
        pub async fn disable_joint_limits(&self) -> Result<()> {
            self.0.disable_joint_limits().await
        }
        #[classmethod]
        pub async fn enable_joint_limits(&self) -> Result<()> {
            self.0.enable_joint_limits().await
        }

        //MOTOR
        #[classmethod]
        pub async fn find_zero(&self) -> Result<()> {
            self.0.find_zero().await
        }

        // KINEMATIC
        #[classmethod]
        #[cmod::tags(ret)]
        pub async fn load_tcp(&self, name: String, dir: Option<String>) -> Result<CartesianPose> {
            self.0.load_tcp(name, dir).await
        }
        #[classmethod]
        #[cmod::tags(args(pose))]
        pub async fn set_tcp(&self, pose: CartesianPose) -> Result<()> {
            self.0.set_tcp(pose).await
        }
        #[classmethod]
        #[cmod::tags(ret)]
        pub async fn get_tcp(&self) -> Result<CartesianPose> {
            self.0.get_tcp().await
        }
        #[classmethod]
        pub async fn set_velocity_factor(&self, speed_factor: i32) -> Result<()> {
            self.0.set_velocity_factor(speed_factor).await
        }
        #[classmethod]
        pub async fn get_velocity_factor(&self) -> Result<i32> {
            self.0.get_velocity_factor().await
        }

        // Status
        #[classmethod]
        #[cmod::tags(ret)]
        pub async fn get_estop_reason(&self) -> Result<EstopReason> {
            self.0.get_estop_reason().await
        }
        #[classmethod]
        #[cmod::tags(ret)]
        pub async fn get_kin_data(&self) -> Result<KinData> {
            self.0.get_kin_data().await
        }
        #[classmethod]
        #[cmod::tags(ret)]
        pub async fn get_robot_state(&self) -> Result<RobotState> {
            self.0.get_robot_state().await
        }
        #[classmethod]
        #[cmod::tags(ret)]
        pub async fn get_phy_data(&self) -> Result<PhyData> {
            self.0.get_phy_data().await
        }

        // DYNAMIC
        #[classmethod]
        #[cmod::tags(ret)]
        pub async fn load_payload(&self, name: String, dir: Option<String>) -> Result<Payload> {
            self.0.load_payload(name, dir).await
        }
        #[classmethod]
        #[cmod::tags(args(cog))]
        pub async fn set_payload(&self, mass: Option<f64>, cog: Option<Position>) -> Result<()> {
            self.0.set_payload(mass, cog).await
        }
        #[classmethod]
        #[cmod::tags(ret)]
        pub async fn get_payload(&self) -> Result<Payload> {
            self.0.get_payload().await
        }
        #[classmethod]
        #[cmod::tags(args(pose))]
        pub async fn set_gravity(&self, pose: Position) -> Result<()> {
            self.0.set_gravity(pose).await
        }
        #[classmethod]
        #[cmod::tags(ret)]
        pub async fn get_gravity(&self) -> Result<Position> {
            self.0.get_gravity().await
        }

        // utils
        #[classmethod]
        #[cmod::tags(args(robot_state))]
        pub async fn can_move(&self, robot_state: RobotState) -> Result<bool> {
            Ok(matches!(robot_state, RobotState::Idle | RobotState::Paused | RobotState::Moving))
        }
        #[classmethod]
        #[cmod::tags(args(p))]
        pub async fn in_pose(&self, p: Pose) -> Result<bool> {
            let ret = match p {
                Pose::Joint(j) => {
                    let diff = 0.0017; //0.1°
                    let pose = self.get_kin_data().await?.target_joint_pose;
                    pose.0.into_iter().zip(j.0).all(|(cur, tar)| (cur - tar).abs() < diff)
                }
                Pose::Cart(c) => {
                    let mut ret = true;
                    let diff_tran = 0.005; //5mm
                    let diff_rot = 0.017; //1°
                    let pose = self.get_kin_data().await?.target_tcp_pose;
                    if (pose.x - c.x).abs() > diff_tran || (pose.y - c.y).abs() > diff_tran || (pose.z - c.z).abs() > diff_tran {
                        ret = false;
                    }
                    if (pose.rx - c.rx).abs() > diff_rot || (pose.ry - c.ry).abs() > diff_rot || (pose.rz - c.rz).abs() > diff_rot {
                        ret = false;
                    }
                    ret
                }
            };
            Ok(ret)
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

pub use lebai_sdk::*;
