"""Lebai 机器人 Python SDK 类型声明与使用说明。

仅 3.1.8 及以上版本的机器人系统可用。仅支持 Python 3.7 及以上版本。

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

安装:

- 常规同步调用方式：``pip install lebai-sdk``（如已安装 lebai-sdk-asyncio 需先卸载）。
- 异步协程调用方式（AsyncIO/async/await）：``pip install lebai-sdk-asyncio``（如已安装 lebai-sdk 需先卸载）。

常用约定:

- 所有接口均为同步阻塞调用，失败时抛出 ``TypeError``（错误信息为字符串）。
- 关节位姿 ``JointPose`` 为 6 元素 ``list[float]``（弧度）。
- 笛卡尔位姿 ``CartesianPose`` 为 ``dict``：位置单位 m，角度单位 rad。
- 枚举类型在 Python 中均为字符串（如机器人状态 ``"IDLE"``、``"MOVING"``）。
- ``ip`` 支持两种格式：
  1. 局域网 IP，如 ``"192.168.2.101"``；
  2. frpc 远程地址，如 ``"user:passwd@22dadd5c5fd0.nodes.lebai.ltd"``。
- 由于通信不稳定等原因可能导致错误发生，大多数情况下建议捕获错误，防止异常退出。
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
    """获取 SDK 的版本号。"""


def init() -> None:
    """初始化 SDK 运行时。导入 SDK 后仅需初始化一次（可选，首次调用时自动初始化）。"""


def timestamp() -> int:
    """获取当前系统时间戳（毫秒）。"""


def sleep_ms(ms: int) -> None:
    """阻塞休眠指定的毫秒数。

    Args:
        ms: 休眠时间（毫秒）。
    """


def discover_devices(time: float) -> list[DeviceInfo]:
    """设备发现（基于 mDNS 技术）：在持续时间内连续发现周边设备，持续时间结束后返回周边所有在线的机器人列表。

    Args:
        time: 扫描时长（秒）。

    Returns:
        设备信息列表，包含 ``name`` 设备名称、``ip`` IP 地址、``mac`` MAC 地址、``online`` 是否在线。
    """


def connect(ip: str, port: bool | int | None = None, option: ConnectOption | None = None) -> Robot:
    """通过 IP 寻址连接到乐白机械臂。

    Args:
        ip: 机器人 IP，支持局域网 IP（如 ``"192.168.2.101"``）或 frpc 远程地址（如 ``"user:passwd@xxx.nodes.lebai.ltd"``）。
        port: 端口选择——``None`` 或 ``False`` 为真实机器人（默认 3031），
            ``True`` 为模拟器（3030），``int`` 为自定义端口。可选，默认 ``None``。
        option: 连接选项，如 ``{"request_timeout": 60}``。可选，默认 ``None``。

    Returns:
        机械臂实例，后续控制机械臂的方法需使用该实例。

    Raises:
        TypeError: 连接失败。
    """


class Robot:
    """机械臂实例。

    连接成功后即可调用各类控制、查询方法。``Robot`` 可被复制，
    多个引用共享同一个连接。
    """

    # ---- 连接状态 ----

    def is_connected(self) -> bool:
        """获取连接状态。

        Returns:
            是否处于连接状态。
        """

    def wait_disconnect(self) -> str:
        """阻塞等待设备断开连接。常用于设备断开后重连。

        Returns:
            设备断开的原因。
        """

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
        """估算关节位姿的可达性（0~1，越大越灵活）。

        在临近奇异位置和工作空间边界的地方灵活性接近于 ``0``。通常，严格条件下小于 0.001
        视为灵活性差，宽松条件下小于 0.0001 视为灵活性差。

        Args:
            p: 关节位姿（弧度）。
        """

    def kinematics_forward(self, p: Pose) -> CartesianPose:
        """正运动学：通过正向运动学将关节角度转成笛卡尔位置和姿态。

        Args:
            p: 位姿参数，关节位姿（list）或笛卡尔位姿（dict）二选一。
        """

    def kinematics_inverse(self, p: Pose, refer: JointPose | None = None) -> JointPose:
        """逆运动学：通过逆向运动学将笛卡尔位置和姿态转换成关节角度。

        计算结果与当前 TCP 设置和当前关节位置有关。

        Args:
            p: 笛卡尔位置和姿态。
            refer: 反解参考的关节位置。可选，默认为当前反馈关节位置；反解出现多解时取距离 ``refer`` 最近的一个解。
        """

    def pose_trans(self, a: Pose, b: Pose) -> CartesianPose:
        """位姿特征坐标系转换：求 ``a`` 系下 ``b`` 的相对位姿。

        Args:
            a: 位姿 A。
            b: 位姿 B。

        Returns:
            位姿 C = AB，即 ``b`` 相对于机器人世界坐标系的位姿描述。
        """

    def pose_add(self, pose: Pose, delta: CartesianPose, frame: CartesianPose | None = None) -> CartesianPose:
        """位姿的加：在 ``pose`` 基础上增加偏移 ``delta``。

        从 base 位置，往 ``frame`` 方向移动 delta 后的位姿。

        Args:
            pose: 起始位姿。
            delta: 位姿偏移量。
            frame: 位姿偏移量方向，仅姿态部分有效。可选，默认机器人基座方向。
        """

    def pose_inverse(self, p: Pose) -> CartesianPose:
        """位姿的逆：求位姿 ``p`` 对应的齐次矩阵的逆的位姿描述，可用于求解位姿方程。

        Args:
            p: 位姿，关节位姿（list）或笛卡尔位姿（dict）二选一。
        """

    def save_pose(self, name: str, pose: Pose | None = None, dir: str | None = None, refer: JointPose | None = None) -> None:
        """保存位姿到机器人存储。

        Args:
            name: 位置名称。
            pose: 待保存的位姿，关节位姿（list）或笛卡尔位姿（dict）二选一；为 ``None`` 时保存当前位置。
            dir: 位置所在文件名称。可选，默认根目录。
            refer: 参考关节位姿。可选，默认 ``None`` 不保存参考关节位姿。
        """

    def load_pose(self, name: str, dir: str | None = None, raw_pose: bool | None = None) -> Pose | None:
        """从资源库读取已保存的位姿。

        Args:
            name: 位置名称。
            dir: 位置所在文件名称。可选，默认根目录。
            raw_pose: 返回的位姿类型。默认 ``False`` 返回关节位姿；设置为 ``True`` 时按保存时的原始格式返回关节位姿或笛卡尔位姿。
        """

    def load_frame(self, name: str, dir: str | None = None) -> CartesianPose:
        """从资源库读取已保存的特征坐标系。

        Args:
            name: 名称。
            dir: 工作空间名称。可选，默认根目录。

        Returns:
            相对于基座的偏移量。
        """

    # ---- 运动 ----

    def pause_move(self) -> None:
        """暂停所有运动。不影响任务运行状态。"""

    def resume_move(self) -> None:
        """恢复所有运动。不影响任务运行状态。"""

    def stop_move(self) -> None:
        """停止所有运动，但无法取消后续新发的运动指令。"""

    def wait_move(self, id: int | None = None) -> None:
        """阻塞等待指定运动完成。

        Args:
            id: 运动 ID；为 ``None`` 时等待全部运动完成。
        """

    def get_running_motion(self) -> int:
        """当前正在执行的运动 ID。"""

    def get_motion_state(self, id: int) -> str:
        """查询运动状态（字符串）。

        Args:
            id: ``move_xxx`` 返回的运动 ID。

        Returns:
            运动状态："WAIT" 排队中；"RUNNING" 运动中；"FINISHED" 已完成。
        """

    def towardj(self, p: Pose, a: float, v: float, t: float | None = None, r: float | None = None) -> int:
        """关节跟随运动：连续发出 ``towardj`` 指令，会忽略前面发出的运动目标位置，以最新指令的位置参数为目标进行运动。

        Args:
            p: 位置参数。``[0.1, 0.2, 0.2, 0.3, 0.1, 0.2]`` 表示关节角表达的关节位置；
                ``{'x': 0.01, 'y': 0, 'z': 0, 'rz': 0, 'ry': 0, 'rx': 0}`` 表示笛卡尔坐标位置。
            a: 运动的关节加速度（rad/s²）。
            v: 运动的关节速度（rad/s）。
            t: 运动时间（s），指定后忽略 a/v。
            r: 运动轨迹交融半径（m），范围 0 到 1。

        Returns:
            运动 ID。
        """

    def movej(self, p: Pose, a: float, v: float, t: float | None = None, r: float | None = None) -> int:
        """关节运动：在关节空间内执行线性运动。

        使用该命令，机器人必须处于静止状态或者上一个命令是 ``movej`` 或带交融的 ``movel``。

        Args:
            p: 位置参数。``[0.1, 0.2, 0.2, 0.3, 0.1, 0.2]`` 表示关节角表达的关节位置；
                ``{'x': 0.01, 'y': 0, 'z': 0, 'rz': 0, 'ry': 0, 'rx': 0}`` 表示笛卡尔坐标位置。
            a: 运动的关节加速度（rad/s²）。
            v: 运动的关节速度（rad/s）。
            t: 运动时间（s），指定后忽略 a/v。
            r: 运动轨迹交融半径（m），范围 0 到 1。

        Returns:
            运动 ID。
        """

    def movel(self, p: Pose, a: float, v: float, t: float | None = None, r: float | None = None) -> int:
        """直线运动：笛卡尔空间直线运动到目标位姿。

        Args:
            p: 位置参数。``[0.1, 0.2, 0.2, 0.3, 0.1, 0.2]`` 表示关节角表达的关节位置（将通过运动学正解转为坐标位置）；
                ``{'x': 0.01, 'y': 0, 'z': 0, 'rz': 0, 'ry': 0, 'rx': 0}`` 表示笛卡尔坐标位置。
            a: 运动的工具空间加速度（m/s²）。
            v: 运动的工具空间速度（m/s）。
            t: 运动时间（s），指定后忽略 a/v。
            r: 运动轨迹交融半径（m），范围 0 到 1。

        Returns:
            运动 ID。
        """

    def movec(self, via: Pose, p: Pose, rad: float, a: float, v: float, t: float | None = None, r: float | None = None) -> int:
        """圆弧运动：路径为以当前位置、``via`` 和 ``p`` 三点组成的唯一圆。

        Args:
            via: 途径位置。
            p: 目标位置。
            rad: 路径圆弧的弧度 (rad)。``rad == 0`` 表示运动到 ``p`` 作为终点；
                ``rad > 0`` 表示走一个圆弧运动轨迹，途径 ``via`` 和 ``p``，并旋转对应的 rad；
                ``rad < 0`` 表示反方向走圆弧运动轨迹，``via`` 和 ``p`` 在此模式下不一定经过。
            a: 工具空间加速度（m/s²）。
            v: 工具空间速度（m/s）。
            t: 运动时间（s），指定后忽略 a/v。
            r: 圆滑过渡半径（m）。

        Returns:
            运动 ID。
        """

    def move_pvat(self, p: JointPose, v: list[float], a: list[float], t: float) -> None:
        """关节空间伺服运动（PVAT）：指定每个关节的速度、加速度。

        Args:
            p: 关节位置。
            v: 每个关节的速度（rad/s），如 ``[0.1, 0.2, 0.2, 0.3, 0.1, 0.2]``。
            a: 每个关节的加速度（rad/s²），如 ``[1, 1, 1, 1, 0.5, 0.5]``。
            t: 运动时间（s）。

        注意：
            - 最小可设置时间为 0.01（10ms），小于该值的路径点会被跳过。
            - 2 个相邻点位之间的发送间隔必须小于 ``t``，超出会导致机器人自动减速停止。
            - 注意传参，速度在前面。
        """

    def move_pvt(self, p: JointPose, v: list[float], t: float) -> None:
        """关节空间伺服运动（PVT）：同 PVAT，由机器人自动估算各个关节的加速度。

        Args:
            p: 关节位置。
            v: 每个关节的速度（rad/s）。
            t: 运动时间（s）。
        """

    def move_pt(self, p: JointPose, t: float) -> None:
        """关节空间伺服运动（PT）：同 PVAT，由机器人自动估算各个关节的速度、加速度。

        Args:
            p: 关节位置。
            t: 运动时间（s）。
        """

    def move_trajectory(self, name: str, dir: str | None = None) -> int:
        """回放运动轨迹：执行已保存的轨迹。

        Args:
            name: 轨迹名称。
            dir: 轨迹所在文件名称。可选，默认根目录。

        Returns:
            运动指令 ID。
        """

    def speedj(self, a: float, v: JointPose, t: float | None = None) -> int:
        """关节匀速运动：不指定位置，各个关节按照速度矢量定速运动。

        Args:
            a: 运动的关节加速度（rad/s²）。
            v: 各个关节速度（rad/s），如 ``[0.1, 0.2, 0.2, 0.3, 0.1, 0.2]``。
            t: 运动时间（s）。可选，默认 t=0，一直运动到限位。

        Returns:
            运动 ID。

        注意：速度参数是列表，代表各个关节的速度设定值，不同于 ``movej`` 的 float 类型。
        """

    def speedl(self, a: float, v: CartesianPose, t: float | None = None, frame: CartesianPose | None = None) -> int:
        """直线匀速运动：在指定参考坐标系下按照速度矢量定速运动到限位。

        Args:
            a: 空间加速度（m/s²）。
            v: 各个方向的速度（m/s），如 ``{'x': 0.01, 'y': 0, 'z': 0, 'rz': 0, 'ry': 0, 'rx': 0}``。
            t: 运动时间（s）。可选，默认 t=0，一直运动到限位。
            frame: 运动坐标参考系。可选，默认基座方向。

        Returns:
            运动 ID。
        """

    def teach_mode(self) -> None:
        """进入示教（自由驱动）模式。仅当机器人处于空闲状态时可调用。

        在示教模式下，机器人各关节可被自由拖拽。末端负载配置会影响示教效果，
        设置值比实际负载大时机器人会向上用力，反之会向下用力。
        """

    def end_teach_mode(self) -> None:
        """退出示教（自由驱动）模式，恢复到空闲状态。处与其他模式时调用不产生错误。"""

    # ---- IO ----

    # IO 设备类型：
    # - "ROBOT"：控制箱，带 4DI/4DO/2AI/2AO
    # - "FLANGE"：法兰盘，带 2DI/2DO
    # - "EXTRA"：扩展板，带 12DI/12DO/2AI/2AO
    # - "SHOULDER"：肩部灯板，带 1DI（已用作按键）
    # - "FLANGE_BTN"：法兰按钮，带 2DI（已用作按键）

    def set_dio_mode(self, device: IoDevice, pin: int, mode: DigitalMode) -> None:
        """设置指定数字 IO 引脚的方向（输入或输出）。

        Args:
            device: 设备类型（"ROBOT"、"FLANGE"、"EXTRA"、"SHOULDER"、"FLANGE_BTN"）。
            pin: 端口，从 0 开始。
            mode: "INPUT" 输入 / "OUTPUT" 输出。
        """

    def get_dio_mode(self, device: IoDevice, pin: int) -> DigitalMode:
        """获取指定数字 IO 引脚的方向（输入或输出）。"""

    def set_do(self, device: IoDevice, pin: int, value: int) -> None:
        """设置对应的 DO 端口打开和关闭。常用于控制电磁阀的开闭。

        Args:
            value: 待设置的值，0 关闭 / 1 打开。
        """

    def get_do(self, device: IoDevice, pin: int) -> int:
        """获取对应的 DO 端口是打开还是关闭的状态。

        Returns:
            0 关闭，1 打开。
        """

    def get_dos(self, device: IoDevice, pin: int, num: int) -> list[int]:
        """获取连续多个 DO 端口是打开还是关闭的状态。

        Args:
            pin: 起始端口号。
            num: 连续数量。
        """

    def get_di(self, device: IoDevice, pin: int) -> int:
        """获取对应的 DI 端口是否有电信号输入。常用于光电传感器的检测。

        Returns:
            0 无信号，1 有信号。
        """

    def get_dis(self, device: IoDevice, pin: int, num: int) -> list[int]:
        """获取连续多个 DI 端口是否有电信号输入。常用于光电传感器的检测。

        Args:
            pin: 起始端口号。
            num: 连续数量。
        """

    def set_ao(self, device: IoDevice, pin: int, value: float) -> None:
        """设置对应的 AO 端口输出设定的电压或者电流。

        Args:
            value: 待设置的值。电压型 0-10V，电流型 4-20mA。

        注：控制界面有设置端口是输出电压还是电流的选择。
        """

    def get_ao(self, device: IoDevice, pin: int) -> float:
        """获取对应的 AO 端口输出的电压或者电流的值。"""

    def get_aos(self, device: IoDevice, pin: int, num: int) -> list[float]:
        """获取连续多个 AO 端口输出的电压或者电流的值。

        Args:
            pin: 起始端口号。
            num: 连续数量。
        """

    def get_ai(self, device: IoDevice, pin: int) -> float:
        """获取对应的 AI 端口输入的电压或者电流的值。"""

    def get_ais(self, device: IoDevice, pin: int, num: int) -> list[float]:
        """获取连续多个 AI 端口输入的电压或者电流的值。

        Args:
            pin: 起始端口号。
            num: 连续数量。
        """

    # ---- 信号量 ----

    # 系统内置 256 个信号量，下标从 0 开始到 255，每个信号量存储一个 32 位整数
    # （取值范围 -2147483648 ~ 2147483647）。系统重启后所有信号量都会被重新置为 0。
    # 信号量主要用于进程间或与外部系统进行通信。

    def set_signal(self, index: int, value: int) -> None:
        """设置信号量。

        Args:
            index: 信号量下标。
            value: 待设置的信号量值。
        """

    def set_signals(self, index: int, values: list[int]) -> None:
        """从起始下标起批量设置信号量。

        Args:
            index: 信号量起始下标。
            values: 待设置的信号量值。
        """

    def get_signal(self, index: int) -> int:
        """获取信号量。

        Args:
            index: 信号量下标。

        Returns:
            对应信号量的值。
        """

    def get_signals(self, index: int, len: int) -> list[int]:
        """从起始下标起连续获取多个信号量。

        Args:
            index: 信号量起始下标。
            len: 待获取的信号量数量。
        """

    def wait_signal(self, index: int, value: int, relation: Relation | None = None) -> None:
        """阻塞等待指定信号量满足条件，满足后立即返回。

        Args:
            index: 信号量下标。
            value: 待比较的值。
            relation: 关系判断操作符。可选，默认 ``"EQ"`` 等于。
                ``EQ`` 等于 / ``NEQ`` 不等于 / ``LT`` 小于 / ``LTE`` 小于等于 / ``GT`` 大于 / ``GTE`` 大于等于。
        """

    def add_signal(self, index: int, value: int) -> None:
        """增加指定下标的信号量值。

        Args:
            index: 信号量下标。
            value: 待增加的信号量值，可为负。

        注意增加值范围，有可能产生溢出。
        """

    # ---- 存储 ----

    # 以字符串的形式存储在机箱中的 Key-Value 键值对。

    def set_item(self, key: str, value: str) -> None:
        """设置键值对。

        Args:
            key: 键名。
            value: 键名对应的值。
        """

    def get_item(self, key: str) -> Item:
        """获取键值对。

        Args:
            key: 键名。
        """

    def get_items(self, prefix: str) -> list[Item]:
        """获取指定前缀的所有键值对。

        Args:
            prefix: 键名前缀。
        """

    # ---- 插件 ----

    def enable_plugin(self, name: str) -> CommandStdout:
        """启用插件。

        Args:
            name: 插件名称。
        """

    def disable_plugin(self, name: str) -> CommandStdout:
        """禁用插件。

        Args:
            name: 插件名称。
        """

    def run_plugin_cmd(self, name: str, params: list[str] | None = None) -> CommandStdout:
        """执行插件命令行程序。

        Args:
            name: 插件名称。
            params: 命令行参数列表。可选，默认空。
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

        串行任务会排队执行，同一时间最多只能运行一个串行任务；并行任务会立即执行，
        可以同时运行多个，环境变量相互独立。被启动的场景包含运动相关接口时务必以串行任务启动。

        Args:
            scene: 场景 ID。
            params: 传递参数，字符串数组。可选，默认空。
            dir: 工作空间名称。可选，默认根目录。
            is_parallel: 是否并列执行。可选，默认串行。
            loop_to: 重复执行次数。可选，默认 1 次，0 代表无限次。

        Returns:
            任务 ID。
        """

    def get_task_list(self) -> list[int]:
        """获取全部未完成的任务 ID。"""

    def get_main_task_id(self) -> int | None:
        """获取串行任务的 ID。无串行任务时返回 ``None``。"""

    def wait_task(self, id: int | None = None) -> str:
        """阻塞等待任务结束，返回任务状态字符串和所有输出。

        Args:
            id: ``start_task`` 返回的 task_id。可选，默认串行任务。
        """

    def get_task_state(self, id: int | None = None) -> str:
        """查询任务状态（字符串）。

        Args:
            id: 任务 ID。可选，默认串行任务。

        Returns:
            任务状态："NONE" 无任务；"WAIT" 排队中；"RUNNING" 运行中；"PAUSE" 暂停中；
            "SUCCESS" 运行成功；"INTERRUPTING" 停止中；"INTERRUPT" 已停止；"FAIL" 运行失败。
        """

    def cancel_task(self, id: int | None = None) -> None:
        """停止指定任务。

        Args:
            id: 任务 ID。可选，默认串行任务。

        当停止串行任务时，会自动调用 ``stop_move`` 停止运动。
        """

    def pause_task(self, id: int | None = None) -> None:
        """暂停指定任务。

        Args:
            id: 任务 ID。可选，默认串行任务。

        当暂停串行任务时，会自动调用 ``pause_move`` 暂停运动。
        """

    def resume_task(self, id: int | None = None) -> None:
        """恢复指定任务。

        Args:
            id: 任务 ID。可选，默认串行任务。

        当恢复串行任务时，会自动调用 ``resume_move`` 恢复运动。
        """

    # ---- 串口 ----

    def set_serial_timeout(self, device: str, timeout: int) -> None:
        """设置串口超时时间。

        Args:
            device: 串口设备地址（485 串口为 ``/dev/ttyS1``，232 串口为 ``/dev/ttyS3``，TTL 串口为 ``/dev/ttyS2``）。
            timeout: 超时时间（毫秒）。默认 800ms。
        """

    def set_serial_baud_rate(self, device: str, baud_rate: int) -> None:
        """设置串口波特率。

        Args:
            device: 串口设备地址。
            baud_rate: 串口波特率。默认 115200。
        """

    def set_serial_parity(self, device: str, parity: Parity) -> None:
        """设置串口校验位。

        Args:
            device: 串口设备地址。
            parity: 校验位——``"None"`` 无校验 / ``"Odd"`` 奇校验 / ``"Even"`` 偶校验。可选，默认无校验。
        """

    def write_serial(self, device: str, data: list[int]) -> None:
        """通过串口发送 u8 数组。

        Args:
            device: 串口设备地址。
            data: 待发送的 u8 数组。
        """

    def read_serial(self, device: str, len: int | None = None) -> list[int]:
        """通过串口接收 u8 数组。

        Args:
            device: 串口设备地址。
            len: 单次接收的最大缓冲长度。可选，默认 64 字节。

        Returns:
            接收到的 u8 数组。
        """

    def clear_serial(self, device: str) -> None:
        """清空串口缓冲区。"""

    # ---- MODBUS ----

    def disconnect_modbus(self, device: str) -> None:
        """断开与指定 Modbus 设备的连接。"""

    def set_modbus_timeout(self, device: str, timeout: int) -> None:
        """设置 Modbus 超时时间。

        Args:
            device: 设备名称。
            timeout: 超时时间（毫秒）。默认 800ms。
        """

    def set_modbus_retry(self, device: str, retry: int) -> None:
        """设置 Modbus 超时重传次数。

        Args:
            device: 设备名称。
            retry: 超时重传次数。默认 0 次。
        """

    def write_single_coil(self, device: str, pin: str, value: bool) -> None:
        """写单个线圈。

        Args:
            device: 设备名称。
            pin: 寄存器地址（字符串格式：例如 ``'0x64'`` 或 ``'100'``）。
            value: 待设置的值。
        """

    def write_multiple_coils(self, device: str, pin: str, values: list[bool]) -> None:
        """写多个线圈。

        Args:
            device: 设备名称。
            pin: 寄存器地址（字符串格式）。
            values: 待设置的值。
        """

    def read_coils(self, device: str, pin: str, count: int) -> list[bool]:
        """读线圈。

        Args:
            device: 设备名称。
            pin: 寄存器地址（字符串格式）。
            count: 连续数量。
        """

    def read_discrete_inputs(self, device: str, pin: str, count: int) -> list[bool]:
        """读离散输入。

        Args:
            device: 设备名称。
            pin: 寄存器地址（字符串格式）。
            count: 连续数量。
        """

    def write_single_register(self, device: str, pin: str, value: int) -> None:
        """写单个寄存器。

        Args:
            device: 设备名称。
            pin: 寄存器地址（字符串格式：例如 ``'0x9C40'`` 或 ``'40000'``）。
            value: 待设置的值。
        """

    def write_multiple_registers(self, device: str, pin: str, values: list[int]) -> None:
        """写多个寄存器。

        Args:
            device: 设备名称。
            pin: 寄存器地址（字符串格式）。
            values: 待设置的值。
        """

    def read_holding_registers(self, device: str, pin: str, count: int) -> list[int]:
        """读取保持寄存器。

        Args:
            device: 设备名称。
            pin: 寄存器地址（字符串格式）。
            count: 连续数量。
        """

    def read_input_registers(self, device: str, pin: str, count: int) -> list[int]:
        """读取输入寄存器。

        Args:
            device: 设备名称。
            pin: 寄存器地址（字符串格式）。
            count: 连续数量。
        """

    # ---- 夹爪 ----

    def init_claw(self, force: bool | None = None) -> None:
        """初始化夹爪，夹爪张合一次以确认行程。需要新版本夹爪硬件支持。

        Args:
            force: 是否强制执行初始化。可选，默认 ``False`` 不强制初始化——监测到夹爪初始化过就不初始化，如果没有则重新初始化一次。

        手臂急停后重新上电，夹爪将会自动张合一次以确认行程。如果夹爪周围有障碍物导致行程确认失真，可以在合适位置强制初始化一次。
        """

    def set_claw(self, force: float | None = None, amplitude: float | None = None) -> None:
        """设置夹爪力度（力控）和幅度（位控），只设置指定项。

        Args:
            force: 力度（0-100）。可选，默认不更改。
            amplitude: 张合幅度（0-100）。可选，默认不更改。

        如果在闭合过程中抓取到物体，则不再继续闭合以避免夹坏物体，判断的准则为设置的力的大小。
        """

    def get_claw(self) -> Claw:
        """读取夹爪状态。"""

    # ---- 法兰 ----

    def set_flange_baud_rate(self, baud_rate: int) -> None:
        """设置法兰串口波特率。

        Args:
            baud_rate: 波特率。默认 115200。
        """

    # ---- LED ----

    def load_led_style(self, name: str, dir: str | None = None) -> LedStyle:
        """从资源库读取已保存的声光样式。

        Args:
            name: 名称。
            dir: 工作空间名称。可选，默认根目录。
        """

    def set_led_style(self, style: LedStyle) -> None:
        """设置声光样式（模式、速度、颜色、声音、音量）。

        Args:
            style: 声光样式。
        """

    def set_led(self, mode: int, speed: int, colors: list[int]) -> None:
        """设置 LED 灯状态。

        Args:
            mode: 亮灯模式。
                - ``0`` 不变
                - ``1`` 关闭
                - ``2`` 常亮
                - ``3`` 呼吸
                - ``4`` 均分旋转
                - ``5`` 同色旋转
                - ``6`` 闪烁
            speed: 速度。
                - ``1`` 快速
                - ``2`` 正常
                - ``3`` 慢速
            colors: 最多包含 4 个 0 ~ 15 之间的整数，对应不同颜色。
        """

    def set_voice(self, voice: int, volume: int) -> None:
        """设置声音。

        Args:
            voice: 声音列表（``0`` 不变；``1``~``10`` 对应不同的提示音，如启动、停止、碰撞告警、示教模式启用/退出、微调等）。
            volume: 音量。
                - ``0`` 静音
                - ``1`` 低
                - ``2`` 正常
                - ``3`` 高
        """

    def set_fan(self, mode: int) -> None:
        """开关风扇。

        Args:
            mode: 风扇模式。``1`` 关闭；``2`` 开启。

        当系统检测到机器人运行温度过高时，会自动开启散热风扇。
        """

    # ---- 系统 ----

    def start_sys(self) -> None:
        """启动手臂：先将机器人上电初始化，再使能关节运动。执行成功后机器人转为空闲状态。

        当机器人处于已急停、已下电、已上电、已停止状态时接受该指令，处于其他状态时该指令不会执行，也不会报错。
        """

    def stop_sys(self) -> None:
        """停止手臂：先停止手臂运动，再禁用关节运动。执行成功后机器人转为已停止状态。

        当机器人处于已上电、空闲、已暂停、运行中、示教中状态时接受该指令，处于其他状态时该指令不会执行，也不会报错。
        """

    def poweron(self) -> None:
        """给机器人手臂上电。与 ``start_sys()`` 不同，该指令只上电，不使能关节运动。"""

    def powerdown(self) -> None:
        """关机：使机器人手臂和控制箱电源断开，同时网络连接也会中断。"""

    def reboot(self) -> None:
        """重启机箱，相当于按下开机按钮关机后，重新按开机按钮开机。"""

    def stop(self) -> None:
        """停止运动：等价于先执行 ``cancel_task``，再执行 ``stop_move``。"""

    def estop(self) -> None:
        """急停（软急停）：立刻断开使机器人手臂下电并刹车，手臂会产生轻微的向下掉落。

        急停后，须调用 ``start_sys()`` 重新给手臂上电。
        """

    # ---- 安全 ----

    def set_collision_detector_sensitivity(self, sensitivity: int) -> None:
        """设置碰撞检测灵敏度，数值越大越灵敏。

        Args:
            sensitivity: 灵敏度，范围 0~100。
        """

    def disable_collision_detector(self) -> None:
        """临时禁用碰撞检测。"""

    def enable_collision_detector(self) -> None:
        """启用碰撞检测。"""

    def disable_joint_limits(self) -> None:
        """临时禁用关节限位。"""

    def enable_joint_limits(self) -> None:
        """启用关节限位。"""

    # ---- 电机 ----

    def find_zero(self) -> None:
        """导轨回零位。仅适用于导轨。"""

    # ---- 运动学 ----

    def load_tcp(self, name: str, dir: str | None = None) -> CartesianPose:
        """从资源库读取已保存的工具中心点（TCP）。

        Args:
            name: 名称。
            dir: 工作空间名称。可选，默认根目录。

        Returns:
            相对于法兰的偏移量。
        """

    def set_tcp(self, pose: CartesianPose) -> None:
        """设置工具中心点（TCP）坐标，坐标值相对于法兰坐标系。仅在运动学上有意义。

        Args:
            pose: 相对于法兰的偏移量。
        """

    def get_tcp(self) -> CartesianPose:
        """获取当前机器人工具中心点（TCP）设置。仅具有运动学意义。"""

    def set_velocity_factor(self, speed_factor: int) -> None:
        """以手臂运动命令的速度为基准，设置实际速度为运动命令速度的对应百分比。

        Args:
            speed_factor: 速度因子百分比，范围 0~100。如 100 表示运动命令速度的 100%，10 表示 10%。
        """

    def get_velocity_factor(self) -> int:
        """获取速度因子。

        Returns:
            速度因子百分比，范围 0~100。
        """

    # ---- 状态查询 ----

    def get_estop_reason(self) -> EstopReason:
        """获取机器人当前急停原因（字符串，如 ``"NONE"``、``"COLLISION"``）。"""

    def get_kin_data(self) -> KinData:
        """读取运动数据（关节/笛卡尔的位置、速度、加速度、力矩）。"""

    def get_robot_state(self) -> RobotState:
        """获取机器人当前状态（如 ``"IDLE"``、``"MOVING"``、``"ESTOP"``）。"""

    def get_phy_data(self) -> PhyData:
        """读取物理数据（温度、电压）。"""

    # ---- 自动配置 ----

    def set_auto(self, name: AutoCfg, value: bool) -> None:
        """设置开机自动配置项。

        重启后生效，设置一次后长期生效。

        Args:
            name: 配置项名称——``"ARM_POWER"`` 开机后自动上电；``"ENABLE_JOINT"`` 开机后自动上电并启动手臂；
                ``"INIT_CLAW"`` 上电后自动初始化夹爪。
            value: true 为使能，false 为禁用。
        """

    def get_auto(self, name: AutoCfg) -> bool | None:
        """获取开机自动配置项。

        Args:
            name: 配置项名称。

        Returns:
            是否启用对应配置项；未设置时返回 ``None``。
        """

    # ---- 动力学 ----

    def load_payload(self, name: str, dir: str | None = None) -> Payload:
        """从资源库读取已保存的末端负载参数。

        Args:
            name: 名称。
            dir: 工作空间名称。可选，默认根目录。
        """

    def set_payload(self, mass: float | None = None, cog: Position | None = None) -> None:
        """设置当前负载参数（只设置指定项）。

        当负载的重量或者重量分布发生变化时，必须调用该方法更新（如机器人捡起或放下较重的物体）。
        该设置会影响拖动示教体验，请务必确保设置与机器人实际情况一致。

        Args:
            mass: 负载质量（kg）。可选，默认不更新。
            cog: 负载重心偏移位置（m），相对于法兰的偏移量。可选，默认不更新。
        """

    def get_payload(self) -> Payload:
        """获取当前机器人末端负载的质量和质心。"""

    def set_gravity(self, pose: Position) -> None:
        """设置机器人重力加速度的方向。

        当机器人固定好后，这个对应的是与地球中心相反的加速方向。该设置会影响拖动示教体验。

        Args:
            pose: 重力方向相对于机器人基座的偏移量。
                正装方向：``{"x": 0, "y": 0, "z": -9.8}``；侧装方向：``{"x": 0, "y": -9.8, "z": 0}``；
                倒装方向：``{"x": 0, "y": 0, "z": 9.8}``。
        """

    def get_gravity(self) -> Position:
        """获取重力方向（相对于机器人基座的偏移量）。"""

    # ---- 网络 ----

    def http(self, req: HttpRequest) -> HttpResponse:
        """由机器人侧代理执行 HTTP 请求。可用于访问机器人机箱所在网络的 HTTP 服务。

        Args:
            req: 请求参数（method、url、headers、body）。
        """

    # ---- 工具 ----

    def can_move(self, robot_state: RobotState) -> bool:
        """根据给定的机器人状态判断机器人是否可运动。

        Args:
            robot_state: 机器人状态。

        Returns:
            ``IDLE``、``PAUSED``、``MOVING`` 状态下为 ``True``，其余状态为 ``False``。
        """

    def in_pose(self, p: Pose) -> bool:
        """判断机器人当前位姿是否在目标位姿附近（关节误差 < 0.1°，位置偏差 < 5mm，角度偏差 < 1°）。

        Args:
            p: 目标位姿，关节位姿（list）或笛卡尔位姿（dict）二选一。
        """


class RobotSubscription:
    """RPC 订阅对象，可反复调用 ``next()`` 获取推送数据。"""

    def next(self) -> str | None:
        """阻塞获取下一条推送数据（JSON 字符串）。

        Returns:
            推送数据；订阅结束时返回 ``None``。
        """
