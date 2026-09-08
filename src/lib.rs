mod common;
#[cfg(feature = "mdns")]
mod mdns;
mod rpc;
mod runtime;

#[cmod::cmod]
mod lebai_sdk {
    use super::*;
    use cmod::Result;
    pub use proto::lebai::auto::AutoCfg;
    pub use proto::lebai::claw::Claw;
    pub use proto::lebai::cmp::Relation;
    pub use proto::lebai::dynamic::Payload;
    pub use proto::lebai::io::DigitalMode;
    pub use proto::lebai::network::{HttpRequest, HttpResponse};
    pub use proto::lebai::posture::Position;
    pub use proto::lebai::system::{EstopReason, PhyData, RobotState};
    pub use proto::serde::kinematic::KinData;
    pub use proto::serde::led::LedStyle;
    pub use proto::serde::posture::{CartesianPose, JointPose, Pose};
    pub use rpc::{ConnectOption, RobotPort};
    use runtime::CompatExt as _;

    /// 获取 SDK 的版本号。
    ///
    /// # 返回
    ///
    /// `String` 版本号
    #[cmod::function]
    pub fn version() -> Result<String> {
        Ok(common::VERSION.into())
    }
    /// 初始化 SDK 运行时。导入 SDK 后仅需初始化一次。
    ///
    /// # 参数
    ///
    /// 无
    ///
    /// # 返回
    ///
    /// 无
    #[cfg(not(target_family = "wasm"))]
    #[cmod::function]
    pub fn init() -> Result<()> {
        #[cfg(feature = "ffi_py")]
        let _ = cmod::ffi::py::init_runtime(&*runtime::RT);

        Ok(())
    }
    /// 获取当前系统时间戳（毫秒）。
    ///
    /// # 返回
    ///
    /// `u64` 毫秒时间戳
    #[cmod::function]
    #[cmod::tags(ret)]
    pub fn timestamp() -> Result<u64> {
        common::timestamp()
    }
    /// 阻塞休眠指定的毫秒数。
    ///
    /// # 参数
    ///
    /// - `ms`: `u64` 休眠时间（毫秒）
    ///
    /// # 返回
    ///
    /// 无
    #[cmod::function]
    #[cmod::tags(args(ms))]
    pub async fn sleep_ms(ms: u64) -> Result<()> {
        common::sleep_ms(ms).compat().await
    }

    /// 设备发现（基于 mDNS 技术）。在持续时间内连续发现周边设备，持续时间结束后返回周边所有在线的机器人列表。
    ///
    /// # 参数
    ///
    /// - `time`: `f64` 扫描时长（秒）
    ///
    /// # 返回
    ///
    /// `Vec<[`DeviceInfo`](proto::lebai::multi_devices::DeviceInfo)>` 设备信息列表，包含 `name` 设备名称、`ip` IP 地址、`mac` MAC 地址、`online` 是否在线
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

    /// 通过 IP 寻址连接到乐白机械臂。支持局域网 IP（如 `"192.168.2.101"`）或 frpc 远程地址（如 `"user:passwd@xxx.nodes.lebai.ltd"`）。
    ///
    /// # 参数
    ///
    /// - `ip`: `String` 乐白机械臂的 IP 地址
    /// - `port`: `Option<`[`RobotPort`]`>` 端口选择——`None` 或 `false` 为真实机器人（默认 3031），`true` 为模拟器（3030），整数为自定义端口。可选，默认 `None`
    /// - `option`: `Option<`[`ConnectOption`]`>` 连接选项，如 `{"request_timeout": 60}`。可选，默认 `None`
    ///
    /// # 返回
    ///
    /// [`Robot`] 机械臂实例，后续控制机械臂的方法需使用该实例
    #[cmod::function]
    #[cmod::tags(args(port, option))]
    pub async fn connect(ip: String, port: Option<RobotPort>, option: Option<ConnectOption>) -> Result<Robot> {
        let robot = rpc::connect(ip, port, option).compat().await?;
        Ok(Robot(robot))
    }

    /// 机械臂实例。连接成功后即可调用各类控制、查询方法。可被复制，多个引用共享同一个连接。
    #[cmod::class]
    #[derive(Clone)]
    pub struct Robot(rpc::Robot);
    #[cmod::methods]
    impl Robot {
        /// 获取连接状态。
        ///
        /// # 返回
        ///
        /// `bool` 是否处于连接状态
        #[classmethod]
        pub async fn is_connected(&self) -> Result<bool> {
            self.0.is_connected().await
        }
        /// 阻塞等待设备断开连接。常用于设备断开后重连。
        ///
        /// # 返回
        ///
        /// `String` 设备断开的原因
        #[classmethod]
        pub async fn wait_disconnect(&self) -> Result<String> {
            self.0.wait_disconnect().await
        }
        /// 调用任意的 JSON-RPC 方法，用于调用 SDK 尚未封装的底层接口。
        ///
        /// # 参数
        ///
        /// - `method`: `String` RPC 方法名
        /// - `param`: `Option<`[`String`]`>` 参数 JSON 字符串。可选，默认 `None`
        ///
        /// # 返回
        ///
        /// `String` 返回结果的 JSON 字符串
        #[classmethod]
        pub async fn call(&self, method: String, param: Option<String>) -> Result<String> {
            self.0.call(method, param).await
        }
        /// 订阅 RPC 方法的推送数据（`sub_<method>` / `unsub_<method>`）。适用于需要实时获取机器人状态、IO 变化等场景。
        ///
        /// # 参数
        ///
        /// - `method`: `String` 订阅方法名
        /// - `param`: `Option<`[`String`]`>` 参数 JSON 字符串。可选，默认 `None`
        ///
        /// # 返回
        ///
        /// [`RobotSubscription`] 订阅对象，可反复调用 `next()` 获取推送数据
        #[classmethod]
        pub async fn subscribe(&self, method: String, param: Option<String>) -> Result<RobotSubscription> {
            let subscription = self.0.subscribe(method, param).await?;
            Ok(RobotSubscription(subscription))
        }

        // Posture
        /// 估算关节位姿的可达性（0~1，越大越灵活）。在临近奇异位置和工作空间边界的地方灵活性接近于 `0`。
        ///
        /// 通常，严格条件下小于 0.001 视为灵活性差，宽松条件下小于 0.0001 视为灵活性差。
        ///
        /// # 参数
        ///
        /// - `p`: [`JointPose`] 关节位姿（弧度）
        ///
        /// # 返回
        ///
        /// `f64` 灵活性，0~1，越大越灵活
        #[classmethod]
        #[cmod::tags(args(p))]
        pub async fn measure_manipulation(&self, p: JointPose) -> Result<f64> {
            self.0.measure_manipulation(p).await
        }
        /// 正解：通过机器人正向运动学将关节角度转成笛卡尔位置和姿态。
        ///
        /// # 参数
        ///
        /// - `p`: [`Pose`] 位姿参数，关节位姿（list）或笛卡尔位姿（dict）二选一
        ///
        /// # 返回
        ///
        /// [`CartesianPose`] 笛卡尔空间位置和姿态
        #[classmethod]
        #[cmod::tags(args(p), ret)]
        pub async fn kinematics_forward(&self, p: Pose) -> Result<CartesianPose> {
            self.0.kinematics_forward(p).await
        }
        /// 反解：通过机器人逆向运动学将笛卡尔位置和姿态转换成关节角度。计算结果与当前 TCP 设置和当前关节位置有关。
        ///
        /// # 参数
        ///
        /// - `p`: [`Pose`] 笛卡尔位置和姿态
        /// - `refer`: `Option<[`JointPose`]>` 反解参考的关节位置。可选，默认为当前反馈关节位置。当反解出现多解时，会取距离 `refer` 最近的一个解
        ///
        /// # 返回
        ///
        /// [`JointPose`] 关节位置
        #[classmethod]
        #[cmod::tags(args(p, refer), ret)]
        pub async fn kinematics_inverse(&self, p: Pose, refer: Option<JointPose>) -> Result<JointPose> {
            self.0.kinematics_inverse(p, refer).await
        }
        /// 位姿特征坐标系转换：求 `from` 系下 `to` 的相对位姿。以 `from` 为用户坐标系 {A}，`to` 是相对于坐标系 {A} 的位姿描述，返回相对于机器人世界坐标系的位姿描述。
        ///
        /// # 参数
        ///
        /// - `from`: [`Pose`] 位姿 A
        /// - `to`: [`Pose`] 位姿 B
        ///
        /// # 返回
        ///
        /// [`CartesianPose`] 位姿 C = AB，是 `to` 相对于机器人世界坐标系的位姿描述
        #[classmethod]
        #[cmod::tags(args(from, to), ret)]
        pub async fn pose_trans(&self, from: Pose, to: Pose) -> Result<CartesianPose> {
            self.0.pose_trans(from, to).await
        }
        /// 位姿的加：在 `pose` 基础上增加偏移 `delta`。从 base 位置，往 frame 方向移动 delta 后的位姿。
        ///
        /// # 参数
        ///
        /// - `pose`: [`Pose`] 起始位姿
        /// - `delta`: [`CartesianPose`] 位姿偏移量
        /// - `frame`: `Option<[`CartesianPose`]>` 位姿偏移量方向，仅姿态部分有效。可选，默认机器人基座方向
        ///
        /// # 返回
        ///
        /// [`CartesianPose`] 移动后的位姿
        #[classmethod]
        #[cmod::tags(args(pose, frame, delta), ret)]
        pub async fn pose_add(&self, pose: Pose, delta: CartesianPose, frame: Option<CartesianPose>) -> Result<CartesianPose> {
            self.0.pose_add(pose, delta, frame).await
        }
        /// 位姿的逆：求位姿 `p` 对应的齐次矩阵 A 的逆的位姿描述，可用于求解位姿方程。
        ///
        /// # 参数
        ///
        /// - `p`: [`Pose`] 位姿，关节位姿（list）或笛卡尔位姿（dict）二选一
        ///
        /// # 返回
        ///
        /// [`CartesianPose`] A 的逆矩阵对应的笛卡尔位置和姿态
        #[classmethod]
        #[cmod::tags(args(p), ret)]
        pub async fn pose_inverse(&self, p: Pose) -> Result<CartesianPose> {
            self.0.pose_inverse(p).await
        }
        /// 保存位姿到机器人存储。`pose` 为 `None` 时保存当前位置。
        ///
        /// # 参数
        ///
        /// - `name`: `String` 位置名称
        /// - `pose`: `Option<[`Pose`]>` 待保存的位姿，关节位姿（list）或笛卡尔位姿（dict）二选一。可选，默认 `None` 保存当前位置
        /// - `dir`: `Option<[`String`]>` 位置所在文件名称。可选，默认根目录
        /// - `refer`: `Option<[`JointPose`]>` 参考关节位姿。可选，默认 `None` 不保存参考关节位姿
        ///
        /// # 返回
        ///
        /// 无
        #[classmethod]
        #[cmod::tags(args(pose, refer))]
        pub async fn save_pose(&self, name: String, pose: Option<Pose>, dir: Option<String>, refer: Option<JointPose>) -> Result<()> {
            self.0.save_pose(name, pose, dir, refer).await
        }
        /// 读取已保存的位姿。
        ///
        /// # 参数
        ///
        /// - `name`: `String` 位置名称
        /// - `dir`: `Option<[`String`]>` 位置所在文件名称。可选，默认根目录
        /// - `raw_pose`: `Option<[`bool`]>` 返回的位姿类型。可选，默认 `false` 返回关节位姿，设置为 `true` 时按保存时的原始格式返回关节位姿或笛卡尔位姿
        ///
        /// # 返回
        ///
        /// `Option<[`Pose`]>` 对应位置库里的关节位置信息
        #[classmethod]
        #[cmod::tags(ret)]
        pub async fn load_pose(&self, name: String, dir: Option<String>, raw_pose: Option<bool>) -> Result<Option<Pose>> {
            self.0.load_pose(name, dir, raw_pose).await
        }
        /// 从资源库读取已保存的特征坐标系。
        ///
        /// # 参数
        ///
        /// - `name`: `String` 名称
        /// - `dir`: `Option<[`String`]>` 工作空间名称。可选，默认根目录
        ///
        /// # 返回
        ///
        /// [`CartesianPose`] 相对于基座的偏移量
        #[classmethod]
        #[cmod::tags(ret)]
        pub async fn load_frame(&self, name: String, dir: Option<String>) -> Result<CartesianPose> {
            self.0.load_frame(name, dir).await
        }

        // Motion
        /// 暂停所有运动。不影响任务运行状态。
        ///
        /// # 返回
        ///
        /// 无
        #[classmethod]
        pub async fn pause_move(&self) -> Result<()> {
            self.0.pause_move().await
        }
        /// 恢复所有运动。不影响任务运行状态。
        ///
        /// # 返回
        ///
        /// 无
        #[classmethod]
        pub async fn resume_move(&self) -> Result<()> {
            self.0.resume_move().await
        }
        /// 停止所有运动，但无法取消后续新发的运动指令。
        ///
        /// # 返回
        ///
        /// 无
        #[classmethod]
        pub async fn stop_move(&self) -> Result<()> {
            self.0.stop_move().await
        }
        /// 阻塞等待指定运动完成。未指定运动 ID 时等待全部运动完成。
        ///
        /// # 参数
        ///
        /// - `id`: `Option<`u32`>` `move_xxx` 返回的运动 ID。可选，默认为全部运动
        ///
        /// # 返回
        ///
        /// 无
        #[classmethod]
        pub async fn wait_move(&self, id: Option<u32>) -> Result<()> {
            self.0.wait_move(id).await
        }
        /// 获取正在执行的运动 ID。
        ///
        /// # 返回
        ///
        /// `u32` 当前正在执行的运动 ID
        #[classmethod]
        pub async fn get_running_motion(&self) -> Result<u32> {
            self.0.get_running_motion().await
        }
        /// 查询运动状态。
        ///
        /// # 参数
        ///
        /// - `id`: `u32` `move_xxx` 返回的运动 ID
        ///
        /// # 返回
        ///
        /// `String` 运动状态："WAIT" 排队中；"RUNNING" 运动中；"FINISHED" 已完成
        #[classmethod]
        pub async fn get_motion_state(&self, id: u32) -> Result<String> {
            self.0.get_motion_state(id).await
        }
        /// 关节跟随运动：连续发出 towardj 运动指令，会忽略前面发出的运动目标位置，以最新指令的位置参数为目标进行运动。
        ///
        /// # 参数
        ///
        /// - `p`: [`Pose`] 位置参数。`[0.1, 0.2, 0.2, 0.3, 0.1, 0.2]` 表示关节角表达的关节位置；`{'x': 0.01, 'y': 0, 'z': 0, 'rz': 0, 'ry': 0, 'rx': 0}` 表示笛卡尔坐标位置
        /// - `a`: `f64` 运动的关节加速度 (rad/s²)
        /// - `v`: `f64` 运动的关节速度 (rad/s)
        /// - `t`: `Option<`f64`>` 运动时间 (s)。当 t > 0 时，参数速度 v 和加速度 a 无效。可选，默认 0
        /// - `r`: `Option<`f64`>` 运动轨迹交融半径 (m)，范围 0 到 1，用于指定路径的平滑效果。可选，默认 0
        ///
        /// # 返回
        ///
        /// `u32` 运动指令 ID
        #[classmethod]
        #[cmod::tags(args(p))]
        pub async fn towardj(&self, p: Pose, a: f64, v: f64, t: Option<f64>, r: Option<f64>) -> Result<u32> {
            self.0.towardj(p, a, v, t, r).await
        }
        /// 关节运动：在关节空间内执行线性运动。使用该命令，机器人必须处于静止状态或者上一个命令是 movej 或者带交融的 movel。
        ///
        /// # 参数
        ///
        /// - `p`: [`Pose`] 位置参数。`[0.1, 0.2, 0.2, 0.3, 0.1, 0.2]` 表示关节角表达的关节位置；`{'x': 0.01, 'y': 0, 'z': 0, 'rz': 0, 'ry': 0, 'rx': 0}` 表示笛卡尔坐标位置
        /// - `a`: `f64` 运动的关节加速度 (rad/s²)
        /// - `v`: `f64` 运动的关节速度 (rad/s)
        /// - `t`: `Option<`f64`>` 运动时间 (s)。当 t > 0 时，参数速度 v 和加速度 a 无效。可选，默认 0
        /// - `r`: `Option<`f64`>` 运动轨迹交融半径 (m)，范围 0 到 1，用于指定路径的平滑效果。可选，默认 0
        ///
        /// # 返回
        ///
        /// `u32` 运动指令 ID
        #[classmethod]
        #[cmod::tags(args(p))]
        pub async fn movej(&self, p: Pose, a: f64, v: f64, t: Option<f64>, r: Option<f64>) -> Result<u32> {
            self.0.movej(p, a, v, t, r).await
        }
        /// 直线运动：在笛卡尔空间内直线运动到目标位姿。
        ///
        /// # 参数
        ///
        /// - `p`: [`Pose`] 位置参数。`[0.1, 0.2, 0.2, 0.3, 0.1, 0.2]` 表示关节角表达的关节位置（将通过运动学正解转为坐标位置）；`{'x': 0.01, 'y': 0, 'z': 0, 'rz': 0, 'ry': 0, 'rx': 0}` 表示笛卡尔坐标位置
        /// - `a`: `f64` 运动的工具空间加速度 (m/s²)
        /// - `v`: `f64` 运动的工具空间速度 (m/s)
        /// - `t`: `Option<`f64`>` 运动时间 (s)。当 t > 0 时，参数速度 v 和加速度 a 无效。可选，默认 0
        /// - `r`: `Option<`f64`>` 运动轨迹交融半径 (m)，范围 0 到 1，用于指定路径的平滑效果。可选，默认 0
        ///
        /// # 返回
        ///
        /// `u32` 运动指令 ID
        #[classmethod]
        #[cmod::tags(args(p))]
        pub async fn movel(&self, p: Pose, a: f64, v: f64, t: Option<f64>, r: Option<f64>) -> Result<u32> {
            self.0.movel(p, a, v, t, r).await
        }
        /// 圆弧运动：在工具空间内进行圆弧运动，路径为以当前位置、`via` 和 `p` 三点组成的唯一圆。
        ///
        /// # 参数
        ///
        /// - `via`: [`Pose`] 途径位置
        /// - `p`: [`Pose`] 目标位置
        /// - `rad`: `f64` 路径圆弧的弧度 (rad)。rad = 0 表示运动到 p 作为终点；rad > 0 表示走一个圆弧运动轨迹，途径 via 和 p，并旋转对应的 rad；rad < 0 表示反方向走圆弧运动轨迹，via 和 p 在此模式下不一定经过
        /// - `a`: `f64` 工具空间加速度 (m/s²)
        /// - `v`: `f64` 工具空间速度 (m/s)
        /// - `t`: `Option<`f64`>` 运动时间 (s)。可选，默认 0
        /// - `r`: `Option<`f64`>` 运动轨迹交融半径 (m)。可选，默认 0
        ///
        /// # 返回
        ///
        /// `u32` 运动指令 ID
        #[classmethod]
        #[cmod::tags(args(via, p))]
        pub async fn movec(&self, via: Pose, p: Pose, rad: f64, a: f64, v: f64, t: Option<f64>, r: Option<f64>) -> Result<u32> {
            self.0.movec(via, p, rad, a, v, t, r).await
        }
        /// 伺服运动（PVAT）：指定每个关节的速度、加速度，让机器人连续地进行伺服运动。
        ///
        /// # 参数
        ///
        /// - `p`: [`JointPose`] 关节位置
        /// - `v`: `Vec<`f64`>` 每个关节的速度 (rad/s)，如 `[0.1, 0.2, 0.2, 0.3, 0.1, 0.2]`
        /// - `a`: `Vec<`f64`>` 每个关节的加速度 (rad/s²)，如 `[1, 1, 1, 1, 0.5, 0.5]`
        /// - `t`: `f64` 运动时间 (s)
        ///
        /// # 返回
        ///
        /// 无
        #[classmethod]
        #[cmod::tags(args(p, v, a))]
        pub async fn move_pvat(&self, p: JointPose, v: Vec<f64>, a: Vec<f64>, t: f64) -> Result<()> {
            self.0.move_pvat(p, v, a, t).await
        }
        /// 伺服运动（PVT）：同 PVAT，由机器人自动估算各个关节的加速度。
        ///
        /// # 参数
        ///
        /// - `p`: [`JointPose`] 关节位置
        /// - `v`: `Vec<`f64`>` 每个关节的速度 (rad/s)
        /// - `t`: `f64` 运动时间 (s)
        ///
        /// # 返回
        ///
        /// 无
        #[classmethod]
        #[cmod::tags(args(p, v))]
        pub async fn move_pvt(&self, p: JointPose, v: Vec<f64>, t: f64) -> Result<()> {
            self.0.move_pvt(p, v, t).await
        }
        /// 伺服运动（PT）：同 PVAT，由机器人自动估算各个关节的速度、加速度。
        ///
        /// # 参数
        ///
        /// - `p`: [`JointPose`] 关节位置
        /// - `t`: `f64` 运动时间 (s)
        ///
        /// # 返回
        ///
        /// 无
        #[classmethod]
        #[cmod::tags(args(p))]
        pub async fn move_pt(&self, p: JointPose, t: f64) -> Result<()> {
            self.0.move_pt(p, t).await
        }
        /// 回放运动轨迹：执行已保存的轨迹。
        ///
        /// # 参数
        ///
        /// - `name`: `String` 轨迹名称
        /// - `dir`: `Option<[`String`]>` 轨迹所在文件名称。可选，默认根目录
        ///
        /// # 返回
        ///
        /// `u32` 运动指令 ID
        #[classmethod]
        pub async fn move_trajectory(&self, name: String, dir: Option<String>) -> Result<u32> {
            self.0.move_trajectory(name, dir).await
        }
        /// 关节匀速运动：不指定位置，各个关节按照速度矢量定速运动，运动多久根据运动时间参数而定，默认 t = 0，一直运动到限位。
        ///
        /// # 参数
        ///
        /// - `a`: `f64` 运动的关节加速度 (rad/s²)
        /// - `v`: [`JointPose`] 各个关节速度 (rad/s)，如 `[0.1, 0.2, 0.2, 0.3, 0.1, 0.2]`
        /// - `t`: `Option<`f64`>` 运动时间 (s)。可选，默认 t = 0，一直运动到限位
        ///
        /// # 返回
        ///
        /// `u32` 运动指令 ID
        #[classmethod]
        #[cmod::tags(args(v))]
        pub async fn speedj(&self, a: f64, v: JointPose, t: Option<f64>) -> Result<u32> {
            self.0.speedj(a, v, t).await
        }
        /// 直线匀速运动：不指定位置，在指定参考坐标系下按照速度矢量定速运动到限位。
        ///
        /// # 参数
        ///
        /// - `a`: `f64` 空间加速度 (m/s²)
        /// - `v`: [`CartesianPose`] 各个方向的速度 (m/s)，如 `{'x': 0.01, 'y': 0, 'z': 0, 'rz': 0, 'ry': 0, 'rx': 0}`
        /// - `t`: `Option<`f64`>` 运动时间 (s)。可选，默认 t = 0，一直运动到限位
        /// - `frame`: `Option<[`CartesianPose`]>` 运动坐标参考系。可选，默认基座方向
        ///
        /// # 返回
        ///
        /// `u32` 运动指令 ID
        #[classmethod]
        #[cmod::tags(args(v, frame))]
        pub async fn speedl(&self, a: f64, v: CartesianPose, t: Option<f64>, frame: Option<CartesianPose>) -> Result<u32> {
            self.0.speedl(a, v, t, frame).await
        }
        /// 进入示教（自由驱动）模式。仅当机器人处于空闲状态时可调用。
        ///
        /// # 返回
        ///
        /// 无
        #[classmethod]
        pub async fn teach_mode(&self) -> Result<()> {
            self.0.start_teach_mode().await
        }
        /// 退出示教（自由驱动）模式，恢复到空闲状态。处与其他模式时调用不产生错误。
        ///
        /// # 返回
        ///
        /// 无
        #[classmethod]
        pub async fn end_teach_mode(&self) -> Result<()> {
            self.0.end_teach_mode().await
        }

        //IO
        /// 设置指定数字 IO 引脚的方向（输入或输出）。
        ///
        /// # 参数
        ///
        /// - `device`: `String` 设备类型（`"ROBOT"`、`"FLANGE"`、`"EXTRA"`、`"SHOULDER"`、`"FLANGE_BTN"`），参见 [`IoDevice`](proto::lebai::io::IoDevice)
        /// - `pin`: `u32` 端口，从 0 开始
        /// - `mode`: [`DigitalMode`] 引脚方向，`INPUT` 输入 / `OUTPUT` 输出
        ///
        /// # 返回
        ///
        /// 无
        #[classmethod]
        #[cmod::tags(args(device, mode))]
        pub async fn set_dio_mode(&self, device: String, pin: u32, mode: DigitalMode) -> Result<()> {
            self.0.set_dio_mode(device, pin, mode).await
        }
        /// 获取指定数字 IO 引脚的方向（输入或输出）。
        ///
        /// # 参数
        ///
        /// - `device`: `String` 设备类型，参见 [`IoDevice`](proto::lebai::io::IoDevice)
        /// - `pin`: `u32` 端口，从 0 开始
        ///
        /// # 返回
        ///
        /// [`DigitalMode`] 引脚方向，`INPUT` 输入 / `OUTPUT` 输出
        #[classmethod]
        #[cmod::tags(args(device), ret)]
        pub async fn get_dio_mode(&self, device: String, pin: u32) -> Result<DigitalMode> {
            self.0.get_dio_mode(device, pin).await
        }
        /// 设置对应的 DO 端口打开和关闭。常用于控制电磁阀的开闭。
        ///
        /// # 参数
        ///
        /// - `device`: `String` 设备类型，参见 [`IoDevice`](proto::lebai::io::IoDevice)
        /// - `pin`: `u32` 端口，从 0 开始
        /// - `value`: `u32` 待设置的值，0 关闭 / 1 打开
        ///
        /// # 返回
        ///
        /// 无
        #[classmethod]
        #[cmod::tags(args(device))]
        pub async fn set_do(&self, device: String, pin: u32, value: u32) -> Result<()> {
            self.0.set_do(device, pin, value).await
        }
        /// 获取对应的 DO 端口是打开还是关闭的状态。
        ///
        /// # 参数
        ///
        /// - `device`: `String` 设备类型，参见 [`IoDevice`](proto::lebai::io::IoDevice)
        /// - `pin`: `u32` 端口，从 0 开始
        ///
        /// # 返回
        ///
        /// `u32` 0 关闭，1 打开
        #[classmethod]
        #[cmod::tags(args(device))]
        pub async fn get_do(&self, device: String, pin: u32) -> Result<u32> {
            self.0.get_do(device, pin).await
        }
        /// 获取对应的连续多个 DO 端口是打开还是关闭的状态。
        ///
        /// # 参数
        ///
        /// - `device`: `String` 设备类型，参见 [`IoDevice`](proto::lebai::io::IoDevice)
        /// - `pin`: `u32` 起始端口号
        /// - `num`: `u32` 连续数量
        ///
        /// # 返回
        ///
        /// `Vec<`u32`>` 连续多个 DO 输出状态
        #[classmethod]
        #[cmod::tags(args(device), ret)]
        pub async fn get_dos(&self, device: String, pin: u32, num: u32) -> Result<Vec<u32>> {
            self.0.get_dos(device, pin, num).await
        }
        /// 获取对应的 DI 端口是否有电信号输入。常用于光电传感器的检测。
        ///
        /// # 参数
        ///
        /// - `device`: `String` 设备类型，参见 [`IoDevice`](proto::lebai::io::IoDevice)
        /// - `pin`: `u32` 端口，从 0 开始
        ///
        /// # 返回
        ///
        /// `u32` 0 无信号，1 有信号
        #[classmethod]
        #[cmod::tags(args(device))]
        pub async fn get_di(&self, device: String, pin: u32) -> Result<u32> {
            self.0.get_di(device, pin).await
        }
        /// 获取对应的连续多个 DI 端口是否有电信号输入。常用于光电传感器的检测。
        ///
        /// # 参数
        ///
        /// - `device`: `String` 设备类型，参见 [`IoDevice`](proto::lebai::io::IoDevice)
        /// - `pin`: `u32` 起始端口号
        /// - `num`: `u32` 连续数量
        ///
        /// # 返回
        ///
        /// `Vec<`u32`>` 连续多个 DI 输入状态
        #[classmethod]
        #[cmod::tags(args(device), ret)]
        pub async fn get_dis(&self, device: String, pin: u32, num: u32) -> Result<Vec<u32>> {
            self.0.get_dis(device, pin, num).await
        }
        /// 设置对应的 AO 端口输出设定的电压或者电流。
        ///
        /// # 参数
        ///
        /// - `device`: `String` 设备类型，参见 [`IoDevice`](proto::lebai::io::IoDevice)
        /// - `pin`: `u32` 端口，从 0 开始
        /// - `value`: `f64` 待设置的值。电压型 0-10V，电流型 4-20mA
        ///
        /// # 返回
        ///
        /// 无
        #[classmethod]
        #[cmod::tags(args(device))]
        pub async fn set_ao(&self, device: String, pin: u32, value: f64) -> Result<()> {
            self.0.set_ao(device, pin, value).await
        }
        /// 获取对应的 AO 端口输出的电压或者电流的值。
        ///
        /// # 参数
        ///
        /// - `device`: `String` 设备类型，参见 [`IoDevice`](proto::lebai::io::IoDevice)
        /// - `pin`: `u32` 端口，从 0 开始
        ///
        /// # 返回
        ///
        /// `f64` AO 端口输出值
        #[classmethod]
        #[cmod::tags(args(device))]
        pub async fn get_ao(&self, device: String, pin: u32) -> Result<f64> {
            self.0.get_ao(device, pin).await
        }
        /// 获取对应的连续多个 AO 端口输出的电压或者电流的值。
        ///
        /// # 参数
        ///
        /// - `device`: `String` 设备类型，参见 [`IoDevice`](proto::lebai::io::IoDevice)
        /// - `pin`: `u32` 起始端口号
        /// - `num`: `u32` 连续数量
        ///
        /// # 返回
        ///
        /// `Vec<`f64`>` 连续多个 AO 端口的输出值
        #[classmethod]
        #[cmod::tags(args(device), ret)]
        pub async fn get_aos(&self, device: String, pin: u32, num: u32) -> Result<Vec<f64>> {
            self.0.get_aos(device, pin, num).await
        }
        /// 获取对应的 AI 端口输入的电压或者电流的值。
        ///
        /// # 参数
        ///
        /// - `device`: `String` 设备类型，参见 [`IoDevice`](proto::lebai::io::IoDevice)
        /// - `pin`: `u32` 端口，从 0 开始
        ///
        /// # 返回
        ///
        /// `f64` 对应 AI 端口的输入值
        #[classmethod]
        #[cmod::tags(args(device))]
        pub async fn get_ai(&self, device: String, pin: u32) -> Result<f64> {
            self.0.get_ai(device, pin).await
        }
        /// 获取对应的连续多个 AI 端口输入的电压或者电流的值。
        ///
        /// # 参数
        ///
        /// - `device`: `String` 设备类型，参见 [`IoDevice`](proto::lebai::io::IoDevice)
        /// - `pin`: `u32` 起始端口号
        /// - `num`: `u32` 连续数量
        ///
        /// # 返回
        ///
        /// `Vec<`f64`>` 连续多个 AI 端口的输入值
        #[classmethod]
        #[cmod::tags(args(device), ret)]
        pub async fn get_ais(&self, device: String, pin: u32, num: u32) -> Result<Vec<f64>> {
            self.0.get_ais(device, pin, num).await
        }

        //SIGNAL
        /// 设置信号量。
        ///
        /// # 参数
        ///
        /// - `index`: `u32` 信号量下标（0~255）
        /// - `value`: `i32` 待设置的信号量值（32 位整数）
        ///
        /// # 返回
        ///
        /// 无
        #[classmethod]
        pub async fn set_signal(&self, index: u32, value: i32) -> Result<()> {
            self.0.set_signal(index, value).await
        }
        /// 从起始下标起批量设置信号量。
        ///
        /// # 参数
        ///
        /// - `index`: `u32` 信号量起始下标
        /// - `values`: `Vec<`i32`>` 待设置的信号量值
        ///
        /// # 返回
        ///
        /// 无
        #[classmethod]
        #[cmod::tags(args(values))]
        pub async fn set_signals(&self, index: u32, values: Vec<i32>) -> Result<()> {
            self.0.set_signals(index, values).await
        }
        /// 获取信号量。
        ///
        /// # 参数
        ///
        /// - `index`: `u32` 信号量下标
        ///
        /// # 返回
        ///
        /// `i32` 对应信号量的值
        #[classmethod]
        pub async fn get_signal(&self, index: u32) -> Result<i32> {
            self.0.get_signal(index).await
        }
        /// 从起始下标起连续获取多个信号量。
        ///
        /// # 参数
        ///
        /// - `index`: `u32` 信号量起始下标
        /// - `len`: `u32` 待获取的信号量数量
        ///
        /// # 返回
        ///
        /// `Vec<`i32`>` 连续多个信号量对应的值
        #[classmethod]
        #[cmod::tags(ret)]
        pub async fn get_signals(&self, index: u32, len: u32) -> Result<Vec<i32>> {
            self.0.get_signals(index, len).await
        }
        /// 阻塞等待指定信号量满足条件，满足后立即返回。
        ///
        /// # 参数
        ///
        /// - `index`: `u32` 信号量下标
        /// - `value`: `i32` 待比较的值
        /// - `relation`: `Option<[`Relation`]>` 关系判断操作符。可选，默认 `EQ` 等于（`EQ` 等于 / `NEQ` 不等于 / `LT` 小于 / `LTE` 小于等于 / `GT` 大于 / `GTE` 大于等于）
        ///
        /// # 返回
        ///
        /// 无
        #[classmethod]
        #[cmod::tags(args(relation))]
        pub async fn wait_signal(&self, index: u32, value: i32, relation: Option<Relation>) -> Result<()> {
            self.0.wait_signal(index, value, relation).await
        }
        /// 增加指定下标的信号量值。注意增加值范围，有可能产生溢出。
        ///
        /// # 参数
        ///
        /// - `index`: `u32` 信号量下标
        /// - `value`: `i32` 待增加的信号量值，可为负
        ///
        /// # 返回
        ///
        /// 无
        #[classmethod]
        pub async fn add_signal(&self, index: u32, value: i32) -> Result<()> {
            self.0.add_signal(index, value).await
        }

        // Storage
        /// 以字符串的形式设置 Key-Value 键值对（存储于机箱）。
        ///
        /// # 参数
        ///
        /// - `key`: `String` 键名
        /// - `value`: `String` 键名对应的值
        ///
        /// # 返回
        ///
        /// 无
        #[classmethod]
        pub async fn set_item(&self, key: String, value: String) -> Result<()> {
            self.0.set_item(key, value).await
        }
        /// 获取指定键的键值对。
        ///
        /// # 参数
        ///
        /// - `key`: `String` 键名
        ///
        /// # 返回
        ///
        /// [`Item`](proto::lebai::storage::Item) 键名对应的值
        #[classmethod]
        #[cmod::tags(ret)]
        pub async fn get_item(&self, key: String) -> Result<proto::lebai::storage::Item> {
            self.0.get_item(key).await
        }
        /// 获取指定前缀的所有键值对。
        ///
        /// # 参数
        ///
        /// - `prefix`: `String` 键名前缀
        ///
        /// # 返回
        ///
        /// `Vec<[`Item`](proto::lebai::storage::Item)>` 键值对数组
        #[classmethod]
        #[cmod::tags(ret)]
        pub async fn get_items(&self, prefix: String) -> Result<Vec<proto::lebai::storage::Item>> {
            self.0.get_items(prefix).await
        }

        //Plugin
        /// 启用插件。
        ///
        /// # 参数
        ///
        /// - `name`: `String` 插件名称
        ///
        /// # 返回
        ///
        /// [`CommandStdout`](proto::lebai::CommandStdout) 命令执行结果
        #[classmethod]
        #[cmod::tags(ret)]
        pub async fn enable_plugin(&self, name: String) -> Result<proto::lebai::CommandStdout> {
            self.0.enable_plugin(name).await
        }
        /// 禁用插件。
        ///
        /// # 参数
        ///
        /// - `name`: `String` 插件名称
        ///
        /// # 返回
        ///
        /// [`CommandStdout`](proto::lebai::CommandStdout) 命令执行结果
        #[classmethod]
        #[cmod::tags(ret)]
        pub async fn disable_plugin(&self, name: String) -> Result<proto::lebai::CommandStdout> {
            self.0.disable_plugin(name).await
        }
        /// 执行插件命令行程序。
        ///
        /// # 参数
        ///
        /// - `name`: `String` 插件名称
        /// - `params`: `Option<`Vec<`String`>`>` 命令行参数列表。可选，默认空
        ///
        /// # 返回
        ///
        /// [`CommandStdout`](proto::lebai::CommandStdout) 命令执行结果
        #[classmethod]
        #[cmod::tags(args(params), ret)]
        pub async fn run_plugin_cmd(&self, name: String, params: Option<Vec<String>>) -> Result<proto::lebai::CommandStdout> {
            self.0.run_plugin_cmd(name, params).await
        }
        //TASK
        /// 启动新任务，等待新任务开始运行后返回新任务的 task_id。
        ///
        /// 串行任务会排队执行，同一时间最多只能运行一个；并行任务会立即执行，可同时运行多个，环境变量相互独立。
        ///
        /// # 参数
        ///
        /// - `scene`: `String` 场景 ID
        /// - `params`: `Option<`Vec<`String`>`>` 传递参数，字符串数组。可选，默认空
        /// - `dir`: `Option<[`String`]>` 工作空间名称。可选，默认根目录
        /// - `is_parallel`: `Option<[`bool`]>` 是否并列执行。可选，默认串行
        /// - `loop_to`: `Option<`u32`>` 重复执行次数。可选，默认 1 次，0 代表无限次
        ///
        /// # 返回
        ///
        /// `u32` 任务 ID
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
        /// 获取全部未完成的任务 ID。
        ///
        /// # 返回
        ///
        /// `Vec<`u32`>` 全部未完成的任务的 ID
        #[classmethod]
        #[cmod::tags(ret)]
        pub async fn get_task_list(&self) -> Result<Vec<u32>> {
            self.0.get_task_list().await
        }
        /// 获取串行任务的 ID。
        ///
        /// # 返回
        ///
        /// `Option<`u32`>` 串行任务的 ID。无串行任务时返回 `None`
        #[classmethod]
        pub async fn get_main_task_id(&self) -> Result<Option<u32>> {
            self.0.get_main_task_id().await
        }
        /// 阻塞等待任务结束，返回任务状态字符串和所有输出。
        ///
        /// # 参数
        ///
        /// - `id`: `Option<`u32`>` `start_task` 返回的 task_id。可选，默认串行任务
        ///
        /// # 返回
        ///
        /// `String` 任务状态字符串
        #[classmethod]
        pub async fn wait_task(&self, id: Option<u32>) -> Result<String> {
            self.0.wait_task(id).await
        }
        /// 获取任务状态。
        ///
        /// # 参数
        ///
        /// - `id`: `Option<`u32`>` 任务 ID。可选，默认串行任务
        ///
        /// # 返回
        ///
        /// `String` 任务状态："NONE" 无任务；"WAIT" 排队中；"RUNNING" 运行中；"PAUSE" 暂停中；"SUCCESS" 运行成功；"INTERRUPTING" 停止中；"INTERRUPT" 已停止；"FAIL" 运行失败
        #[classmethod]
        pub async fn get_task_state(&self, id: Option<u32>) -> Result<String> {
            self.0.get_task_state(id).await
        }
        /// 停止指定任务。当停止串行任务时，会自动调用 `stop_move` 停止运动。
        ///
        /// # 参数
        ///
        /// - `id`: `Option<`u32`>` 任务 ID。可选，默认串行任务
        ///
        /// # 返回
        ///
        /// 无
        #[classmethod]
        pub async fn cancel_task(&self, id: Option<u32>) -> Result<()> {
            self.0.cancel_task(id).await
        }
        /// 暂停指定任务。当暂停串行任务时，会自动调用 `pause_move` 暂停运动。
        ///
        /// # 参数
        ///
        /// - `id`: `Option<`u32`>` 任务 ID。可选，默认串行任务
        ///
        /// # 返回
        ///
        /// 无
        #[classmethod]
        pub async fn pause_task(&self, id: Option<u32>) -> Result<()> {
            self.0.pause_task(id).await
        }
        /// 恢复指定任务。当恢复串行任务时，会自动调用 `resume_move` 恢复运动。
        ///
        /// # 参数
        ///
        /// - `id`: `Option<`u32`>` 任务 ID。可选，默认串行任务
        ///
        /// # 返回
        ///
        /// 无
        #[classmethod]
        pub async fn resume_task(&self, id: Option<u32>) -> Result<()> {
            self.0.resume_task(id).await
        }

        // Serial
        /// 设置串口超时时间。
        ///
        /// # 参数
        ///
        /// - `device`: `String` 串口设备地址
        /// - `timeout`: `u32` 超时时间，单位毫秒(ms)。默认 800ms
        ///
        /// # 返回
        ///
        /// 无
        #[classmethod]
        pub async fn set_serial_timeout(&self, device: String, timeout: u32) -> Result<()> {
            self.0.set_serial_timeout(device, timeout).await
        }
        /// 设置串口波特率。
        ///
        /// # 参数
        ///
        /// - `device`: `String` 串口设备地址
        /// - `baud_rate`: `u32` 串口波特率。默认 115200
        ///
        /// # 返回
        ///
        /// 无
        #[classmethod]
        pub async fn set_serial_baud_rate(&self, device: String, baud_rate: u32) -> Result<()> {
            self.0.set_serial_baud_rate(device, baud_rate).await
        }
        /// 设置串口奇偶校验位。
        ///
        /// # 参数
        ///
        /// - `device`: `String` 串口设备地址
        /// - `parity`: [`Parity`](proto::lebai::serial::Parity) 奇偶校验位（`None`: 无奇偶校验；`Odd`: 奇校验；`Even`: 偶校验）。可选，默认无奇偶校验位
        ///
        /// # 返回
        ///
        /// 无
        #[classmethod]
        #[cmod::tags(args(parity))]
        pub async fn set_serial_parity(&self, device: String, parity: proto::lebai::serial::Parity) -> Result<()> {
            self.0.set_serial_parity(device, parity).await
        }
        /// 通过串口发送 u8 数组。
        ///
        /// # 参数
        ///
        /// - `device`: `String` 串口设备地址
        /// - `data`: `Vec<`u8`>` 待发送的 u8 数组
        ///
        /// # 返回
        ///
        /// 无
        #[classmethod]
        #[cmod::tags(args(data))]
        pub async fn write_serial(&self, device: String, data: Vec<u8>) -> Result<()> {
            self.0.write_serial(device, data).await
        }
        /// 通过串口接收 u8 数组。
        ///
        /// # 参数
        ///
        /// - `device`: `String` 串口设备地址
        /// - `len`: `Option<`u32`>` 单次接收的最大缓冲长度。可选，默认 64 字节
        ///
        /// # 返回
        ///
        /// `Vec<`u8`>` 接收到的 u8 数组
        #[classmethod]
        #[cmod::tags(ret)]
        pub async fn read_serial(&self, device: String, len: Option<u32>) -> Result<Vec<u8>> {
            self.0.read_serial(device, len).await
        }
        /// 清除串口收发缓存的数据。
        ///
        /// # 参数
        ///
        /// - `device`: `String` 串口设备地址
        ///
        /// # 返回
        ///
        /// 无
        #[classmethod]
        pub async fn clear_serial(&self, device: String) -> Result<()> {
            self.0.clear_serial(device).await
        }

        //MODBUS
        /// 断开与指定 Modbus 设备的连接。
        ///
        /// # 参数
        ///
        /// - `device`: `String` 设备名称
        ///
        /// # 返回
        ///
        /// 无
        #[classmethod]
        pub async fn disconnect_modbus(&self, device: String) -> Result<()> {
            self.0.disconnect_modbus(device).await
        }
        /// 设置 Modbus 超时时间。
        ///
        /// # 参数
        ///
        /// - `device`: `String` 设备名称
        /// - `timeout`: `u32` 超时时间，单位毫秒(ms)。默认 800ms
        ///
        /// # 返回
        ///
        /// 无
        #[classmethod]
        pub async fn set_modbus_timeout(&self, device: String, timeout: u32) -> Result<()> {
            self.0.set_modbus_timeout(device, timeout).await
        }
        /// 设置 Modbus 超时重传次数。
        ///
        /// # 参数
        ///
        /// - `device`: `String` 设备名称
        /// - `retry`: `u32` 超时重传次数。默认 0 次
        ///
        /// # 返回
        ///
        /// 无
        #[classmethod]
        pub async fn set_modbus_retry(&self, device: String, retry: u32) -> Result<()> {
            self.0.set_modbus_retry(device, retry).await
        }
        /// 写单个线圈。
        ///
        /// # 参数
        ///
        /// - `device`: `String` 设备名称
        /// - `pin`: `String` 寄存器地址（字符串格式：例如 `'0x64'` 或者 `'100'`）
        /// - `value`: `bool` 待设置的值
        ///
        /// # 返回
        ///
        /// 无
        #[classmethod]
        pub async fn write_single_coil(&self, device: String, pin: String, value: bool) -> Result<()> {
            self.0.write_single_coil(device, pin, value).await
        }
        /// 写多个线圈。
        ///
        /// # 参数
        ///
        /// - `device`: `String` 设备名称
        /// - `pin`: `String` 寄存器地址（字符串格式：例如 `'0x64'` 或者 `'100'`）
        /// - `values`: `Vec<`bool`>` 待设置的值
        ///
        /// # 返回
        ///
        /// 无
        #[classmethod]
        #[cmod::tags(args(values))]
        pub async fn write_multiple_coils(&self, device: String, pin: String, values: Vec<bool>) -> Result<()> {
            self.0.write_multiple_coils(device, pin, values).await
        }
        /// 读线圈。
        ///
        /// # 参数
        ///
        /// - `device`: `String` 设备名称
        /// - `pin`: `String` 寄存器地址（字符串格式：例如 `'0x64'` 或者 `'100'`）
        /// - `count`: `u32` 连续数量
        ///
        /// # 返回
        ///
        /// `Vec<`bool`>` 连续线圈的状态
        #[classmethod]
        #[cmod::tags(ret)]
        pub async fn read_coils(&self, device: String, pin: String, count: u32) -> Result<Vec<bool>> {
            self.0.read_coils(device, pin, count).await
        }
        /// 读离散输入。
        ///
        /// # 参数
        ///
        /// - `device`: `String` 设备名称
        /// - `pin`: `String` 寄存器地址（字符串格式：例如 `'0x64'` 或者 `'100'`）
        /// - `count`: `u32` 连续数量
        ///
        /// # 返回
        ///
        /// `Vec<`bool`>` 连续的离散寄存器数值的状态
        #[classmethod]
        #[cmod::tags(ret)]
        pub async fn read_discrete_inputs(&self, device: String, pin: String, count: u32) -> Result<Vec<bool>> {
            self.0.read_discrete_inputs(device, pin, count).await
        }
        /// 写单个寄存器。
        ///
        /// # 参数
        ///
        /// - `device`: `String` 设备名称
        /// - `pin`: `String` 寄存器地址（字符串格式：例如 `'0x9C40'` 或者 `'40000'`）
        /// - `value`: `u32` 待设置的值
        ///
        /// # 返回
        ///
        /// 无
        #[classmethod]
        pub async fn write_single_register(&self, device: String, pin: String, value: u32) -> Result<()> {
            self.0.write_single_register(device, pin, value).await
        }
        /// 写多个寄存器。
        ///
        /// # 参数
        ///
        /// - `device`: `String` 设备名称
        /// - `pin`: `String` 寄存器地址（字符串格式：例如 `'0x9C40'` 或者 `'40000'`）
        /// - `values`: `Vec<`u32`>` 待设置的值
        ///
        /// # 返回
        ///
        /// 无
        #[classmethod]
        #[cmod::tags(args(values))]
        pub async fn write_multiple_registers(&self, device: String, pin: String, values: Vec<u32>) -> Result<()> {
            self.0.write_multiple_registers(device, pin, values).await
        }
        /// 读取保持寄存器。
        ///
        /// # 参数
        ///
        /// - `device`: `String` 设备名称
        /// - `pin`: `String` 寄存器地址（字符串格式：例如 `'0x9C40'` 或者 `'40000'`）
        /// - `count`: `u32` 连续数量
        ///
        /// # 返回
        ///
        /// `Vec<`u32`>` 对应的连续保持寄存器地址位对应的数值
        #[classmethod]
        #[cmod::tags(ret)]
        pub async fn read_holding_registers(&self, device: String, pin: String, count: u32) -> Result<Vec<u32>> {
            self.0.read_holding_registers(device, pin, count).await
        }
        /// 读取输入寄存器。
        ///
        /// # 参数
        ///
        /// - `device`: `String` 设备名称
        /// - `pin`: `String` 寄存器地址（字符串格式：例如 `'0x9C40'` 或者 `'40000'`）
        /// - `count`: `u32` 连续数量
        ///
        /// # 返回
        ///
        /// `Vec<`u32`>` 对应的连续输入寄存器地址位对应的数值
        #[classmethod]
        #[cmod::tags(ret)]
        pub async fn read_input_registers(&self, device: String, pin: String, count: u32) -> Result<Vec<u32>> {
            self.0.read_input_registers(device, pin, count).await
        }

        //CLAW
        /// 初始化夹爪，夹爪张合一次以确认行程。需要新版本夹爪硬件支持。
        ///
        /// # 参数
        ///
        /// - `force`: `Option<[`bool`]>` 是否强制执行初始化。可选，默认 `false` 不强制初始化设置夹爪，监测到夹爪初始化过就不初始化，如果没有则重新初始化一次
        ///
        /// # 返回
        ///
        /// 无
        #[classmethod]
        pub async fn init_claw(&self, force: Option<bool>) -> Result<()> {
            self.0.init_claw(force).await
        }
        /// 设置夹爪力度（力控）和幅度（位控）。如果在闭合过程中抓取到物体，则不再继续闭合以避免夹坏物体，判断的准则为设置的力的大小。
        ///
        /// # 参数
        ///
        /// - `force`: `Option<`f64`>` 力度（0-100）。可选，默认不更改
        /// - `amplitude`: `Option<`f64`>` 张合幅度（0-100）。可选，默认不更改
        ///
        /// # 返回
        ///
        /// 无
        #[classmethod]
        pub async fn set_claw(&self, force: Option<f64>, amplitude: Option<f64>) -> Result<()> {
            self.0.set_claw(force, amplitude).await
        }
        /// 获取夹爪状态。
        ///
        /// # 返回
        ///
        /// [`Claw`] 当前夹爪状态
        #[classmethod]
        #[cmod::tags(ret)]
        pub async fn get_claw(&self) -> Result<Claw> {
            self.0.get_claw().await
        }

        //Flange
        /// 设置法兰串口波特率。
        ///
        /// # 参数
        ///
        /// - `baud_rate`: `u32` 波特率。默认 115200
        ///
        /// # 返回
        ///
        /// 无
        #[classmethod]
        pub async fn set_flange_baud_rate(&self, baud_rate: u32) -> Result<()> {
            self.0.set_flange_baud_rate(baud_rate).await
        }

        //LED
        /// 读取已保存的 LED 声光样式。
        ///
        /// # 参数
        ///
        /// - `name`: `String` 名称
        /// - `dir`: `Option<[`String`]>` 工作空间名称。可选，默认根目录
        ///
        /// # 返回
        ///
        /// [`LedStyle`] 声光样式
        #[classmethod]
        #[cmod::tags(ret)]
        pub async fn load_led_style(&self, name: String, dir: Option<String>) -> Result<LedStyle> {
            self.0.load_led_style(name, dir).await
        }
        /// 设置 LED 声光样式（模式、速度、颜色、声音、音量）。
        ///
        /// # 参数
        ///
        /// - `style`: [`LedStyle`] 声光样式
        ///
        /// # 返回
        ///
        /// 无
        #[classmethod]
        #[cmod::tags(args(style))]
        pub async fn set_led_style(&self, style: LedStyle) -> Result<()> {
            self.0.set_led_style(style).await
        }
        /// 设置 LED 灯状态。
        ///
        /// # 参数
        ///
        /// - `mode`: `i32` 亮灯模式。`0` 不变；`1` 关闭；`2` 常亮；`3` 呼吸；`4` 均分旋转；`5` 同色旋转；`6` 闪烁
        /// - `speed`: `i32` 速度。`1` 快速；`2` 正常；`3` 慢速
        /// - `colors`: `Vec<`i32`>` 最多包含 4 个 0 ~ 15 之间的整数，对应不同颜色
        ///
        /// # 返回
        ///
        /// 无
        #[classmethod]
        pub async fn set_led(&self, mode: i32, speed: i32, colors: Vec<i32>) -> Result<()> {
            self.0.set_led(mode, speed, colors).await
        }
        /// 设置声音。
        ///
        /// # 参数
        ///
        /// - `voice`: `i32` 声音列表（0~10，对应不同的提示音）
        /// - `volume`: `i32` 音量。`0` 静音；`1` 低；`2` 正常；`3` 高
        ///
        /// # 返回
        ///
        /// 无
        #[classmethod]
        pub async fn set_voice(&self, voice: i32, volume: i32) -> Result<()> {
            self.0.set_voice(voice, volume).await
        }
        /// 开关风扇。
        ///
        /// # 参数
        ///
        /// - `mode`: `i32` 风扇模式。`1` 关闭；`2` 开启
        ///
        /// # 返回
        ///
        /// 无
        #[classmethod]
        pub async fn set_fan(&self, mode: i32) -> Result<()> {
            self.0.set_fan(mode).await
        }

        //SYSTEM
        /// 启动手臂：先将机器人上电初始化，再使能关节运动。执行成功后机器人转为空闲状态。
        ///
        /// 当机器人处于已急停、已下电、已上电、已停止状态时接受该指令，处于其他状态时该指令不会执行，也不会报错。
        ///
        /// # 返回
        ///
        /// 无
        #[classmethod]
        pub async fn start_sys(&self) -> Result<()> {
            self.0.start_sys().await
        }
        /// 停止手臂：先停止手臂运动，再禁用关节运动。执行成功后机器人转为已停止状态。
        ///
        /// 当机器人处于已上电、空闲、已暂停、运行中、示教中状态时接受该指令，处于其他状态时该指令不会执行，也不会报错。
        ///
        /// # 返回
        ///
        /// 无
        #[classmethod]
        pub async fn stop_sys(&self) -> Result<()> {
            self.0.stop_sys().await
        }
        /// 给机器人手臂上电。与 `start_sys()` 不同，该指令只上电，不使能关节运动。
        ///
        /// # 返回
        ///
        /// 无
        #[classmethod]
        pub async fn poweron(&self) -> Result<()> {
            self.0.poweron().await
        }
        /// 关机：使机器人手臂和控制箱电源断开，同时网络连接也会中断。
        ///
        /// # 返回
        ///
        /// 无
        #[classmethod]
        pub async fn powerdown(&self) -> Result<()> {
            self.0.powerdown().await
        }
        /// 重启机箱。
        ///
        /// # 返回
        ///
        /// 无
        #[classmethod]
        pub async fn reboot(&self) -> Result<()> {
            self.0.reboot().await
        }
        /// 停止运动：等价于先执行 `cancel_task`，再执行 `stop_move`。
        ///
        /// # 返回
        ///
        /// 无
        #[classmethod]
        pub async fn stop(&self) -> Result<()> {
            self.0.stop().await
        }
        /// 急停（软急停）：立刻断开使机器人手臂下电并刹车，手臂会产生轻微的向下掉落。急停后，须调用 `start_sys()` 重新给手臂上电。
        ///
        /// # 返回
        ///
        /// 无
        #[classmethod]
        pub async fn estop(&self) -> Result<()> {
            self.0.estop().await
        }

        //Safety
        /// 设置碰撞检测灵敏度，数值越大越灵敏。
        ///
        /// # 参数
        ///
        /// - `sensitivity`: `u32` 灵敏度，范围 0~100
        ///
        /// # 返回
        ///
        /// 无
        #[classmethod]
        pub async fn set_collision_detector_sensitivity(&self, sensitivity: u32) -> Result<()> {
            self.0.set_collision_detector_sensitivity(sensitivity).await
        }
        /// 临时禁用碰撞检测。
        ///
        /// # 返回
        ///
        /// 无
        #[classmethod]
        pub async fn disable_collision_detector(&self) -> Result<()> {
            self.0.disable_collision_detector().await
        }
        /// 启用碰撞检测。
        ///
        /// # 返回
        ///
        /// 无
        #[classmethod]
        pub async fn enable_collision_detector(&self) -> Result<()> {
            self.0.enable_collision_detector().await
        }
        /// 临时禁用关节限位。
        ///
        /// # 返回
        ///
        /// 无
        #[classmethod]
        pub async fn disable_joint_limits(&self) -> Result<()> {
            self.0.disable_joint_limits().await
        }
        /// 启用关节限位。
        ///
        /// # 返回
        ///
        /// 无
        #[classmethod]
        pub async fn enable_joint_limits(&self) -> Result<()> {
            self.0.enable_joint_limits().await
        }

        //MOTOR
        /// 导轨找零，仅适用于导轨。
        ///
        /// # 返回
        ///
        /// 无
        #[classmethod]
        pub async fn find_zero(&self) -> Result<()> {
            self.0.find_zero().await
        }

        // KINEMATIC
        /// 读取已保存的工具中心点（TCP）。
        ///
        /// # 参数
        ///
        /// - `name`: `String` 名称
        /// - `dir`: `Option<[`String`]>` 工作空间名称。可选，默认根目录
        ///
        /// # 返回
        ///
        /// [`CartesianPose`] 相对于法兰的偏移量
        #[classmethod]
        #[cmod::tags(ret)]
        pub async fn load_tcp(&self, name: String, dir: Option<String>) -> Result<CartesianPose> {
            self.0.load_tcp(name, dir).await
        }
        /// 设置工具中心点（TCP）坐标，坐标值相对于法兰坐标系。仅在运动学上有意义。
        ///
        /// # 参数
        ///
        /// - `pose`: [`CartesianPose`] 相对于法兰的偏移量
        ///
        /// # 返回
        ///
        /// 无
        #[classmethod]
        #[cmod::tags(args(pose))]
        pub async fn set_tcp(&self, pose: CartesianPose) -> Result<()> {
            self.0.set_tcp(pose).await
        }
        /// 获取当前机器人工具中心点（TCP）设置。仅具有运动学意义。
        ///
        /// # 返回
        ///
        /// [`CartesianPose`] 当前机器人 TCP
        #[classmethod]
        #[cmod::tags(ret)]
        pub async fn get_tcp(&self) -> Result<CartesianPose> {
            self.0.get_tcp().await
        }
        /// 以手臂运动命令的速度为基准，设置实际速度为运动命令速度的对应百分比。
        ///
        /// # 参数
        ///
        /// - `speed_factor`: `i32` 速度因子百分比，范围 0~100
        ///
        /// # 返回
        ///
        /// 无
        #[classmethod]
        pub async fn set_velocity_factor(&self, speed_factor: i32) -> Result<()> {
            self.0.set_velocity_factor(speed_factor).await
        }
        /// 获取速度因子。
        ///
        /// # 返回
        ///
        /// `i32` 速度因子百分比，范围 0~100
        #[classmethod]
        pub async fn get_velocity_factor(&self) -> Result<i32> {
            self.0.get_velocity_factor().await
        }

        // Status
        /// 获取机器人当前急停原因。
        ///
        /// # 返回
        ///
        /// [`EstopReason`] 急停原因
        #[classmethod]
        #[cmod::tags(ret)]
        pub async fn get_estop_reason(&self) -> Result<EstopReason> {
            self.0.get_estop_reason().await
        }
        /// 读取运动数据（关节/笛卡尔的位置、速度、加速度、力矩）。
        ///
        /// # 返回
        ///
        /// [`KinData`] 运动数据
        #[classmethod]
        #[cmod::tags(ret)]
        pub async fn get_kin_data(&self) -> Result<KinData> {
            self.0.get_kin_data().await
        }
        /// 获取机器人当前状态。
        ///
        /// # 返回
        ///
        /// [`RobotState`] 当前机器人状态
        #[classmethod]
        #[cmod::tags(ret)]
        pub async fn get_robot_state(&self) -> Result<RobotState> {
            self.0.get_robot_state().await
        }
        /// 读取物理数据（温度、电压）。
        ///
        /// # 返回
        ///
        /// [`PhyData`] 物理数据
        #[classmethod]
        #[cmod::tags(ret)]
        pub async fn get_phy_data(&self) -> Result<PhyData> {
            self.0.get_phy_data().await
        }

        // AUTO
        /// 设置开机自动配置项。重启后生效，设置一次后长期生效。
        ///
        /// # 参数
        ///
        /// - `name`: [`AutoCfg`] 自动配置项名称（`ARM_POWER` 开机后自动上电 / `ENABLE_JOINT` 开机后自动上电并启动手臂 / `INIT_CLAW` 上电后自动初始化夹爪）
        /// - `value`: `bool` true 为使能，false 为禁用
        ///
        /// # 返回
        ///
        /// 无
        #[classmethod]
        #[cmod::tags(args(name))]
        pub async fn set_auto(&self, name: AutoCfg, value: bool) -> Result<()> {
            self.0.set_auto(name, value).await
        }
        /// 获取自动配置项。
        ///
        /// # 参数
        ///
        /// - `name`: [`AutoCfg`] 自动配置项名称
        ///
        /// # 返回
        ///
        /// `Option<[`bool`]>` 是否启用对应配置项；未设置时返回 `None`
        #[classmethod]
        #[cmod::tags(args(name), ret)]
        pub async fn get_auto(&self, name: AutoCfg) -> Result<Option<bool>> {
            self.0.get_auto(name).await
        }

        // DYNAMIC
        /// 读取已保存的末端负载参数。
        ///
        /// # 参数
        ///
        /// - `name`: `String` 名称
        /// - `dir`: `Option<[`String`]>` 工作空间名称。可选，默认根目录
        ///
        /// # 返回
        ///
        /// [`Payload`] 末端负载
        #[classmethod]
        #[cmod::tags(ret)]
        pub async fn load_payload(&self, name: String, dir: Option<String>) -> Result<Payload> {
            self.0.load_payload(name, dir).await
        }
        /// 设置当前负载参数（只设置指定项）。当负载的重量或者重量分布发生变化时，必须调用该方法更新。
        ///
        /// # 参数
        ///
        /// - `mass`: `Option<`f64`>` 负载质量（kg）。可选，默认不更新
        /// - `cog`: `Option<[`Position`]>` 负载重心偏移位置（m），相对于法兰的偏移量。可选，默认不更新
        ///
        /// # 返回
        ///
        /// 无
        #[classmethod]
        #[cmod::tags(args(cog))]
        pub async fn set_payload(&self, mass: Option<f64>, cog: Option<Position>) -> Result<()> {
            self.0.set_payload(mass, cog).await
        }
        /// 获取当前机器人末端负载的质量和质心。
        ///
        /// # 返回
        ///
        /// [`Payload`] 当前机器人末端负载
        #[classmethod]
        #[cmod::tags(ret)]
        pub async fn get_payload(&self) -> Result<Payload> {
            self.0.get_payload().await
        }
        /// 设置机器人重力加速度的方向。当机器人固定好后，这个对应的是与地球中心相反的加速方向。
        ///
        /// # 参数
        ///
        /// - `pose`: [`Position`] 重力方向相对于机器人的基座的偏移量。正装方向：`{"x": 0, "y": 0, "z": -9.8}`；侧装方向：`{"x": 0, "y": -9.8, "z": 0}`；倒装方向：`{"x": 0, "y": 0, "z": 9.8}`
        ///
        /// # 返回
        ///
        /// 无
        #[classmethod]
        #[cmod::tags(args(pose))]
        pub async fn set_gravity(&self, pose: Position) -> Result<()> {
            self.0.set_gravity(pose).await
        }
        /// 获取重力方向。
        ///
        /// # 返回
        ///
        /// [`Position`] 重力方向相对于机器人的基座的偏移量
        #[classmethod]
        #[cmod::tags(ret)]
        pub async fn get_gravity(&self) -> Result<Position> {
            self.0.get_gravity().await
        }

        // Network
        /// 由机器人侧代理执行 HTTP 请求。可用于访问机器人机箱所在网络的 HTTP 服务。
        ///
        /// # 参数
        ///
        /// - `req`: [`HttpRequest`] HTTP 请求
        ///
        /// # 返回
        ///
        /// [`HttpResponse`] HTTP 响应
        #[classmethod]
        #[cmod::tags(args(req), ret)]
        pub async fn http(&self, req: HttpRequest) -> Result<HttpResponse> {
            self.0.http(req).await
        }

        // utils
        /// 根据给定的机器人状态判断机器人是否可运动。
        ///
        /// # 参数
        ///
        /// - `robot_state`: [`RobotState`] 机器人状态
        ///
        /// # 返回
        ///
        /// `bool` 是否可运动。`IDLE`、`PAUSED`、`MOVING` 状态下为 `true`，其余状态为 `false`
        #[classmethod]
        #[cmod::tags(args(robot_state))]
        pub async fn can_move(&self, robot_state: RobotState) -> Result<bool> {
            Ok(matches!(robot_state, RobotState::Idle | RobotState::Paused | RobotState::Moving))
        }
        /// 判断机器人当前位姿是否在目标位姿附近（关节误差 < 0.1°，位置偏差 < 5mm，角度偏差 < 1°）。
        ///
        /// # 参数
        ///
        /// - `p`: [`Pose`] 目标位姿，关节位姿（list）或笛卡尔位姿（dict）二选一
        ///
        /// # 返回
        ///
        /// `bool` 是否已到达目标位姿
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
    /// RPC 订阅对象，可反复调用 `next()` 获取推送数据。
    #[cmod::class]
    #[derive(Clone)]
    pub struct RobotSubscription(rpc::RobotSubscription);
    #[cmod::methods]
    impl RobotSubscription {
        /// 获取下一条推送数据。
        ///
        /// # 返回
        ///
        /// `Option<`[`String`]`>` 推送数据的 JSON 字符串；订阅结束时返回 `None`
        #[classmethod]
        pub async fn next(&self) -> Result<Option<String>> {
            self.0.next().await
        }
    }
}

pub use lebai_sdk::*;
