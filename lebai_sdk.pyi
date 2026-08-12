"""Lebai 机器人 Python SDK 类型声明与使用说明。

快速开始::

    import lebai_sdk

    lebai = lebai_sdk.connect("192.168.2.101")          # 连接真实机器人（默认 3031 端口）
    lebai = lebai_sdk.connect("192.168.2.101", True)    # 连接模拟器（3030 端口）
    lebai = lebai_sdk.connect("192.168.2.101", 3032)    # 自定义端口

    lebai.start_sys()
    lebai.poweron()
    lebai.movej([0, -0.7854, 1.57, -0.7854, 1.57, 0], 0, 0, 3)  # 关节运动
    data = lebai.get_kin_data()                                  # 运动数据
    print(data["target_tcp_pose"])

常用约定:

- 所有接口均为同步阻塞调用，失败时抛出 ``TypeError``（错误信息为字符串）。
- 关节位姿 ``JointPose`` 为 6 元素 ``list[float]``（弧度）。
- 笛卡尔位姿 ``CartesianPose`` 为 ``dict``：位置单位 m，角度单位 rad。
- 枚举类型在 Python 中均为字符串（如机器人状态 ``"IDLE"``、``"MOVING"``）。
- ``ip`` 支持两种格式：
  1. 局域网 IP，如 ``"192.168.2.101"``；
  2. frpc 远程地址，如 ``"user:passwd@22dadd5c5fd0.nodes.lebai.ltd"``。
"""

from typing import Literal, TypedDict

# ---- 数据类型 ----

#: 关节位姿（弧度）：[关节1, 关节2, ..., 关节6]
JointPose = list[float]


class Position(TypedDict):
    """空间位置（单位 m）。"""
    x: float
    y: float
    z: float


class CartesianPose(TypedDict):
    """笛卡尔位姿（位置单位 m，角度单位 rad）。"""
    x: float
    y: float
    z: float
    rx: float
    ry: float
    rz: float


#: 位姿参数：关节位姿（list）或笛卡尔位姿（dict）二选一
Pose = JointPose | CartesianPose


class DeviceInfo(TypedDict):
    """设备发现返回的设备信息。"""
    name: str
    mac: str
    ip: str
    online: bool


class KinData(TypedDict):
    """运动数据（当前状态）。"""
    actual_joint_pose: JointPose      #: 当前关节位置（弧度）
    actual_joint_speed: JointPose    #: 当前关节速度（rad/s）
    actual_joint_acc: JointPose      #: 当前关节加速度（rad/s²）
    actual_joint_torque: JointPose   #: 当前关节力矩（Nm）
    target_joint_pose: JointPose     #: 目标关节位置（弧度）
    target_joint_speed: JointPose    #: 目标关节速度（rad/s）
    target_joint_acc: JointPose      #: 目标关节加速度（rad/s²）
    target_joint_torque: JointPose   #: 目标关节力矩（Nm）
    actual_tcp_pose: CartesianPose   #: 当前 TCP 位姿
    target_tcp_pose: CartesianPose   #: 目标 TCP 位姿
    actual_flange_pose: CartesianPose  #: 当前法兰位姿


class PhyData(TypedDict):
    """物理数据。"""
    joint_temp: list[float]        #: 关节温度（℃）
    joint_voltage: list[float]     #: 关节电压（V）
    flange_voltage: float          #: 法兰电压（V）


class Payload(TypedDict):
    """负载参数。"""
    mass: float      #: 负载质量（kg）
    cog: Position    #: 负载重心偏移位置（m）


class Claw(TypedDict):
    """夹爪状态。"""
    force: float        #: 当前力度
    amplitude: float    #: 当前开度
    weight: float       #: 负载重量（已弃用）
    hold_on: bool       #: 开度是否稳定


class LedStyle(TypedDict):
    """LED 灯带样式。"""
    mode: int          #: 模式
    speed: int         #: 速度
    colors: list[int]  #: RGB 颜色列表
    voice: int         #: 声音
    volume: int        #: 音量


class CommandStdout(TypedDict):
    """命令执行结果。"""
    done: bool      #: 未开始或已完成
    stdout: str     #: 调试信息
    stderr: str     #: 警告信息
    code: int       #: 返回码，非 0 为失败


class Item(TypedDict):
    """KV 存储项。"""
    key: str
    value: str


class HttpRequest(TypedDict):
    """HTTP 请求。"""
    method: str                    #: 请求方法，如 "GET"、"POST"
    url: str                       #: 请求地址
    headers: dict[str, str]        #: 请求头
    body: str                      #: 请求体


class HttpResponse(TypedDict):
    """HTTP 响应。"""
    status: int                    #: 状态码
    headers: dict[str, str]        #: 响应头
    body: str                      #: 响应体


class ConnectOption(TypedDict):
    """连接选项。"""
    request_timeout: int           #: 请求超时时间（秒）


# ---- 枚举类型（Python 中均为字符串） ----

#: IO 设备名（未知设备名默认视为 "ROBOT"）
IoDevice = Literal["ROBOT", "FLANGE", "EXTRA", "SHOULDER", "FLANGE_BTN"]

#: 数字 IO 模式
DigitalMode = Literal["INPUT", "OUTPUT"]

#: 信号比较关系
Relation = Literal["EQ", "NEQ", "LT", "LTE", "GT", "GTE"]

#: 串口校验位（"None" 表示无校验）
Parity = Literal["None", "Odd", "Even"]

#: 开机自动配置项
AutoCfg = Literal["ARM_POWER", "ENABLE_JOINT", "INIT_CLAW"]

#: 机器人状态
RobotState = Literal[
    "DISCONNECTED",  # 已断开连接
    "ESTOP",         # 急停停止状态
    "BOOTING",       # 启动中
    "ROBOT_OFF",     # 电源关闭
    "ROBOT_ON",      # 电源开启
    "IDLE",          # 空闲中
    "PAUSED",        # 已暂停
    "MOVING",        # 机器人运动中
    "UPDATING",      # 更新固件中
    "STARTING",      # 启动中
    "STOPPING",      # 停止中
    "TEACHING",      # 示教中
    "STOP",          # 普通停止
]

#: 急停原因
EstopReason = Literal[
    "NONE",              # 未急停
    "SYSTEM",            # 软件
    "MANUAL",            # 手动调用接口
    "HARD_ESTOP",        # 硬急停按钮
    "COLLISION",         # 碰撞
    "JOINT_LIMIT",       # 关节限位
    "EXCEED",            # 超速
    "TRAJECTORY_ERROR",  # 运动规划错误
    "COMM_ERROR",        # 通信故障
    "CAN_ERROR",         # CAN 通信故障
    "JOINT_ERROR",       # 关节故障
]


# ---- 模块级接口 ----

def version() -> str:
    """SDK 版本号。"""


def init() -> None:
    """初始化运行时（可选，首次调用时自动初始化）。"""


def timestamp() -> int:
    """系统时间戳（毫秒）。"""


def sleep_ms(ms: int) -> None:
    """阻塞休眠指定毫秒数。"""


def discover_devices(time: float) -> list[DeviceInfo]:
    """局域网设备发现，返回在线机器人列表。

    Args:
        time: 扫描时长（秒）。
    """


def connect(ip: str, port: bool | int | None = None, option: ConnectOption | None = None) -> Robot:
    """连接机器人。

    Args:
        ip: 机器人 IP，支持局域网 IP 或 frpc 远程地址。
        port: 端口选择——``None`` 或 ``False`` 为真实机器人（默认 3031），
            ``True`` 为模拟器（3030），``int`` 为自定义端口。
        option: 连接选项，如 ``{"request_timeout": 60}``。

    Returns:
        机器人实例。

    Raises:
        TypeError: 连接失败。
    """


class Robot:
    """机器人控制接口。

    连接成功后即可调用各类控制、查询方法。``Robot`` 可被复制，
    多个引用共享同一个连接。
    """

    # ---- 连接状态 ----

    def is_connected(self) -> bool:
        """是否保持连接。"""

    def wait_disconnect(self) -> str:
        """阻塞等待连接断开，返回断开原因。"""

    def call(self, method: str, param: str | None = None) -> str:
        """调用任意 RPC 方法（底层透传）。

        Args:
            method: RPC 方法名。
            param: 参数 JSON 字符串。

        Returns:
            返回结果的 JSON 字符串。
        """

    def subscribe(self, method: str, param: str | None = None) -> RobotSubscription:
        """订阅 RPC 方法推送（``sub_<method>`` / ``unsub_<method>``）。

        Args:
            method: 订阅方法名。
            param: 参数 JSON 字符串。

        Returns:
            订阅对象，可反复调用 ``next()`` 获取推送数据。
        """

    # ---- 位置和姿态 ----

    def measure_manipulation(self, p: JointPose) -> float:
        """计算关节位姿的可达性（0~1，越大越灵活）。"""

    def kinematics_forward(self, p: Pose) -> CartesianPose:
        """正运动学：关节位姿 -> 笛卡尔位姿。"""

    def kinematics_inverse(self, p: Pose, refer: JointPose | None = None) -> JointPose:
        """逆运动学：笛卡尔位姿 -> 关节位姿。

        Args:
            refer: 参考关节位姿（多解时选择最接近者）。
        """

    def pose_trans(self, a: Pose, b: Pose) -> CartesianPose:
        """位姿变换：求 ``a`` 系下 ``b`` 的相对位姿。"""

    def pose_add(self, pose: Pose, delta: CartesianPose, frame: CartesianPose | None = None) -> CartesianPose:
        """位姿叠加：在 ``pose`` 基础上增加偏移 ``delta``。

        Args:
            frame: 偏移参考系，默认机器人基座系。
        """

    def pose_inverse(self, p: Pose) -> CartesianPose:
        """位姿求逆。"""

    def save_pose(self, name: str, pose: Pose | None = None, dir: str | None = None, refer: JointPose | None = None) -> None:
        """保存位姿到机器人存储。

        Args:
            name: 位姿名称。
            pose: 位姿；为 ``None`` 时保存当前位置。
            dir: 存储目录。
            refer: 参考关节位姿。
        """

    def load_pose(self, name: str, dir: str | None = None, raw_pose: bool | None = None) -> Pose | None:
        """读取已保存的位姿。

        Args:
            raw_pose: 为 ``True`` 时按原始格式返回。
        """

    def load_frame(self, name: str, dir: str | None = None) -> CartesianPose:
        """读取已保存的坐标系。"""

    # ---- 运动 ----

    def pause_move(self) -> None:
        """暂停当前运动。"""

    def resume_move(self) -> None:
        """继续当前运动。"""

    def stop_move(self) -> None:
        """停止当前运动。"""

    def wait_move(self, id: int | None = None) -> None:
        """阻塞等待指定运动完成。

        Args:
            id: 运动 ID；为 ``None`` 时等待全部运动完成。
        """

    def get_running_motion(self) -> int:
        """当前正在执行的运动 ID。"""

    def get_motion_state(self, id: int) -> str:
        """查询运动状态（字符串，如 ``"IDLE"``、``"MOVING"``）。"""

    def towardj(self, p: Pose, a: float, v: float, t: float | None = None, r: float | None = None) -> int:
        """向关节位姿运动（不保持位姿）。

        Args:
            a: 加速度（rad/s²）。
            v: 速度（rad/s）。
            t: 运动时间（s），指定后忽略 a/v。
            r: 圆滑过渡半径。

        Returns:
            运动 ID。
        """

    def movej(self, p: Pose, a: float, v: float, t: float | None = None, r: float | None = None) -> int:
        """关节空间运动到目标位姿。

        Args:
            p: 目标位姿（关节或笛卡尔）。
            a: 加速度（rad/s²）。
            v: 速度（rad/s）。
            t: 运动时间（s），指定后忽略 a/v。
            r: 圆滑过渡半径。

        Returns:
            运动 ID。
        """

    def movel(self, p: Pose, a: float, v: float, t: float | None = None, r: float | None = None) -> int:
        """笛卡尔空间直线运动到目标位姿。

        Args:
            p: 目标位姿（关节或笛卡尔）。
            a: 加速度（m/s²）。
            v: 速度（m/s）。
            t: 运动时间（s），指定后忽略 a/v。
            r: 圆滑过渡半径。

        Returns:
            运动 ID。
        """

    def movec(self, via: Pose, p: Pose, rad: float, a: float, v: float, t: float | None = None, r: float | None = None) -> int:
        """圆弧运动。

        Args:
            via: 圆弧经过点。
            p: 圆弧终点。
            rad: 圆弧半径（m）。
            a: 加速度（m/s²）。
            v: 速度（m/s）。
            t: 运动时间（s），指定后忽略 a/v。
            r: 圆滑过渡半径。

        Returns:
            运动 ID。
        """

    def move_pvat(self, p: JointPose, v: list[float], a: list[float], t: float) -> None:
        """关节空间 PVA 轨迹（位置-速度-加速度-时间）。"""

    def move_pvt(self, p: JointPose, v: list[float], t: float) -> None:
        """关节空间 PVT 轨迹（位置-速度-时间）。"""

    def move_pt(self, p: JointPose, t: float) -> None:
        """关节空间 PT 轨迹（位置-时间）。"""

    def move_trajectory(self, name: str, dir: str | None = None) -> int:
        """执行已保存的轨迹。

        Args:
            name: 轨迹名称。

        Returns:
            运动 ID。
        """

    def speedj(self, a: float, v: JointPose, t: float | None = None) -> int:
        """关节空间速度运动。

        Args:
            a: 加速度（rad/s²）。
            v: 目标关节速度（rad/s）。
            t: 运动时长（s）。

        Returns:
            运动 ID。
        """

    def speedl(self, a: float, v: CartesianPose, t: float | None = None, frame: CartesianPose | None = None) -> int:
        """笛卡尔空间速度运动。

        Args:
            a: 加速度（m/s²）。
            v: 目标 TCP 速度。
            t: 运动时长（s）。
            frame: 参考坐标系。

        Returns:
            运动 ID。
        """

    def teach_mode(self) -> None:
        """进入示教模式。"""

    def end_teach_mode(self) -> None:
        """退出示教模式。"""

    # ---- IO ----

    def set_dio_mode(self, device: IoDevice, pin: int, mode: DigitalMode) -> None:
        """设置数字 IO 引脚方向（输入/输出）。

        Args:
            device: IO 设备名（"ROBOT"、"FLANGE"、"EXTRA"、"SHOULDER"、"FLANGE_BTN"）。
            pin: 引脚号。
            mode: "INPUT" 或 "OUTPUT"。
        """

    def get_dio_mode(self, device: IoDevice, pin: int) -> DigitalMode:
        """查询数字 IO 引脚方向。"""

    def set_do(self, device: IoDevice, pin: int, value: int) -> None:
        """设置数字输出（0 或 1）。"""

    def get_do(self, device: IoDevice, pin: int) -> int:
        """读取数字输出。"""

    def get_dos(self, device: IoDevice, pin: int, num: int) -> list[int]:
        """批量读取数字输出。

        Args:
            num: 从 ``pin`` 起连续读取的引脚数量。
        """

    def get_di(self, device: IoDevice, pin: int) -> int:
        """读取数字输入。"""

    def get_dis(self, device: IoDevice, pin: int, num: int) -> list[int]:
        """批量读取数字输入。"""

    def set_ao(self, device: IoDevice, pin: int, value: float) -> None:
        """设置模拟输出（0~10V）。"""

    def get_ao(self, device: IoDevice, pin: int) -> float:
        """读取模拟输出。"""

    def get_aos(self, device: IoDevice, pin: int, num: int) -> list[float]:
        """批量读取模拟输出。"""

    def get_ai(self, device: IoDevice, pin: int) -> float:
        """读取模拟输入。"""

    def get_ais(self, device: IoDevice, pin: int, num: int) -> list[float]:
        """批量读取模拟输入。"""

    # ---- 信号量 ----

    def set_signal(self, index: int, value: int) -> None:
        """设置信号量。"""

    def set_signals(self, index: int, values: list[int]) -> None:
        """从 ``index`` 起批量设置信号量。"""

    def get_signal(self, index: int) -> int:
        """读取信号量。"""

    def get_signals(self, index: int, len: int) -> list[int]:
        """从 ``index`` 起批量读取信号量。"""

    def wait_signal(self, index: int, value: int, relation: Relation | None = None) -> None:
        """阻塞等待信号量满足条件。

        Args:
            relation: 比较关系（默认 ``"EQ"`` 等于）。
        """

    def add_signal(self, index: int, value: int) -> None:
        """信号量加 ``value``（可为负）。"""

    # ---- 存储 ----

    def set_item(self, key: str, value: str) -> None:
        """写入 KV 数据。"""

    def get_item(self, key: str) -> Item:
        """读取 KV 数据。"""

    def get_items(self, prefix: str) -> list[Item]:
        """读取指定前缀的全部 KV 数据。"""

    # ---- 插件 ----

    def enable_plugin(self, name: str) -> CommandStdout:
        """启用插件。"""

    def disable_plugin(self, name: str) -> CommandStdout:
        """禁用插件。"""

    def run_plugin_cmd(self, name: str, params: list[str] | None = None) -> CommandStdout:
        """执行插件命令行程序。

        Args:
            params: 命令行参数列表。
        """

    # ---- 任务 ----

    def start_task(
        self,
        scene: str,
        params: list[str] | None = None,
        dir: str | None = None,
        is_parallel: bool | None = None,
        loop_to: int | None = None,
    ) -> int:
        """启动任务。

        Args:
            scene: 任务场景名称。
            params: 场景参数列表。
            dir: 任务目录。
            is_parallel: 是否并行执行。
            loop_to: 循环次数。

        Returns:
            任务 ID。
        """

    def get_task_list(self) -> list[int]:
        """当前任务 ID 列表。"""

    def get_main_task_id(self) -> int | None:
        """主任务 ID。"""

    def wait_task(self, id: int | None = None) -> str:
        """阻塞等待任务结束，返回任务状态字符串。"""

    def get_task_state(self, id: int | None = None) -> str:
        """查询任务状态（字符串）。"""

    def cancel_task(self, id: int | None = None) -> None:
        """取消任务。"""

    def pause_task(self, id: int | None = None) -> None:
        """暂停任务。"""

    def resume_task(self, id: int | None = None) -> None:
        """继续任务。"""

    # ---- 串口 ----

    def set_serial_timeout(self, device: str, timeout: int) -> None:
        """设置串口超时（ms）。"""

    def set_serial_baud_rate(self, device: str, baud_rate: int) -> None:
        """设置串口波特率。"""

    def set_serial_parity(self, device: str, parity: Parity) -> None:
        """设置串口校验位（"None"、"Odd"、"Even"）。"""

    def write_serial(self, device: str, data: list[int]) -> None:
        """写串口数据（字节列表）。"""

    def read_serial(self, device: str, len: int | None = None) -> list[int]:
        """读串口数据，返回字节列表。"""

    def clear_serial(self, device: str) -> None:
        """清空串口缓冲区。"""

    # ---- MODBUS ----

    def disconnect_modbus(self, device: str) -> None:
        """断开 MODBUS 设备。"""

    def set_modbus_timeout(self, device: str, timeout: int) -> None:
        """设置 MODBUS 超时（ms）。"""

    def set_modbus_retry(self, device: str, retry: int) -> None:
        """设置 MODBUS 重试次数。"""

    def write_single_coil(self, device: str, pin: str, value: bool) -> None:
        """写单个线圈。"""

    def write_multiple_coils(self, device: str, pin: str, values: list[bool]) -> None:
        """写多个线圈。"""

    def read_coils(self, device: str, pin: str, count: int) -> list[bool]:
        """读线圈。"""

    def read_discrete_inputs(self, device: str, pin: str, count: int) -> list[bool]:
        """读离散输入。"""

    def write_single_register(self, device: str, pin: str, value: int) -> None:
        """写单个寄存器。"""

    def write_multiple_registers(self, device: str, pin: str, values: list[int]) -> None:
        """写多个寄存器。"""

    def read_holding_registers(self, device: str, pin: str, count: int) -> list[int]:
        """读保持寄存器。"""

    def read_input_registers(self, device: str, pin: str, count: int) -> list[int]:
        """读输入寄存器。"""

    # ---- 夹爪 ----

    def init_claw(self, force: bool | None = None) -> None:
        """初始化夹爪。

        Args:
            force: 是否强制初始化。
        """

    def set_claw(self, force: float | None = None, amplitude: float | None = None) -> None:
        """设置夹爪力度和开度（只设置指定项）。"""

    def get_claw(self) -> Claw:
        """读取夹爪状态。"""

    # ---- 法兰 ----

    def set_flange_baud_rate(self, baud_rate: int) -> None:
        """设置法兰串口波特率。"""

    # ---- LED ----

    def load_led_style(self, name: str, dir: str | None = None) -> LedStyle:
        """读取已保存的 LED 样式。"""

    def set_led_style(self, style: LedStyle) -> None:
        """设置 LED 样式（模式、速度、颜色、声音、音量）。"""

    def set_led(self, mode: int, speed: int, colors: list[int]) -> None:
        """设置 LED 灯带。

        Args:
            mode: 模式。
            speed: 速度。
            colors: RGB 颜色列表。
        """

    def set_voice(self, voice: int, volume: int) -> None:
        """设置语音。

        Args:
            voice: 语音编号。
            volume: 音量。
        """

    def set_fan(self, mode: int) -> None:
        """设置风扇模式。"""

    # ---- 系统 ----

    def start_sys(self) -> None:
        """启动系统（初始化硬件）。"""

    def stop_sys(self) -> None:
        """停止系统。"""

    def poweron(self) -> None:
        """机器人上电。"""

    def powerdown(self) -> None:
        """机器人下电。"""

    def reboot(self) -> None:
        """重启系统。"""

    def stop(self) -> None:
        """停止机器人（普通停止）。"""

    def estop(self) -> None:
        """急停。"""

    # ---- 安全 ----

    def set_collision_detector_sensitivity(self, sensitivity: int) -> None:
        """设置碰撞检测灵敏度。"""

    def disable_collision_detector(self) -> None:
        """关闭碰撞检测。"""

    def enable_collision_detector(self) -> None:
        """开启碰撞检测。"""

    def disable_joint_limits(self) -> None:
        """关闭关节限位。"""

    def enable_joint_limits(self) -> None:
        """开启关节限位。"""

    # ---- 电机 ----

    def find_zero(self) -> None:
        """电机找零。"""

    # ---- 运动学 ----

    def load_tcp(self, name: str, dir: str | None = None) -> CartesianPose:
        """读取已保存的工具中心点（TCP）。"""

    def set_tcp(self, pose: CartesianPose) -> None:
        """设置工具中心点（TCP）。"""

    def get_tcp(self) -> CartesianPose:
        """读取当前工具中心点（TCP）。"""

    def set_velocity_factor(self, speed_factor: int) -> None:
        """设置速度因子（0~100）。"""

    def get_velocity_factor(self) -> int:
        """读取速度因子。"""

    # ---- 状态查询 ----

    def get_estop_reason(self) -> EstopReason:
        """读取急停原因（字符串，如 ``"NONE"``、``"COLLISION"``）。"""

    def get_kin_data(self) -> KinData:
        """读取运动数据（关节/笛卡尔的位置、速度、加速度、力矩）。"""

    def get_robot_state(self) -> RobotState:
        """读取机器人状态（如 ``"IDLE"``、``"MOVING"``、``"ESTOP"``）。"""

    def get_phy_data(self) -> PhyData:
        """读取物理数据（温度、电压）。"""

    # ---- 自动配置 ----

    def set_auto(self, name: AutoCfg, value: bool) -> None:
        """设置开机自动配置项。

        Args:
            name: 配置项（"ARM_POWER"、"ENABLE_JOINT"、"INIT_CLAW"）。
            value: 是否启用。
        """

    def get_auto(self, name: AutoCfg) -> bool | None:
        """读取开机自动配置项。"""

    # ---- 动力学 ----

    def load_payload(self, name: str, dir: str | None = None) -> Payload:
        """读取已保存的负载参数。"""

    def set_payload(self, mass: float | None = None, cog: Position | None = None) -> None:
        """设置当前负载参数（只设置指定项）。

        Args:
            mass: 负载质量（kg）。
            cog: 负载重心偏移（m）。
        """

    def get_payload(self) -> Payload:
        """读取当前负载参数。"""

    def set_gravity(self, pose: Position) -> None:
        """设置当前地心引力方向。"""

    def get_gravity(self) -> Position:
        """读取当前地心引力方向。"""

    # ---- 网络 ----

    def http(self, req: HttpRequest) -> HttpResponse:
        """发起 HTTP 请求（机器人侧代理执行）。

        Args:
            req: 请求参数（method、url、headers、body）。
        """

    # ---- 工具 ----

    def can_move(self, robot_state: RobotState) -> bool:
        """机器人是否可运动（空闲/暂停/运动中）。"""

    def in_pose(self, p: Pose) -> bool:
        """当前位姿是否在目标位姿附近（关节误差 < 0.1°，位置 < 5mm，角度 < 1°）。"""


class RobotSubscription:
    """RPC 订阅对象。"""

    def next(self) -> str | None:
        """阻塞获取下一条推送数据（JSON 字符串）。

        Returns:
            推送数据；订阅结束时返回 ``None``。
        """
