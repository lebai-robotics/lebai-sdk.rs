---
name: lebai-sdk
description: 使用 Python SDK 调用 Lebai 协作机器人（机械臂）
version: 1.0.0
trigger: lebai,cobot,乐白,机器人,机械臂,夹爪,运动,移动
risk: medium
permissions: device_exec
delegate_preference: hybrid
requires_board: false
approval_level: confirm
cooldown_seconds: 0
scheduler_template: none
category: 自动化 - 机器人
---

# Lebai 机器人 Python SDK 使用

## 适用场景

- 需要编写或修改调用 Lebai 协作机器人 Python API 的代码（连接、运动控制、IO、信号量、任务、夹爪、MODBUS、串口、LED 等）。
- 需要解释已有 Python SDK 代码的含义，或排查调用报错。
- 需要确认某个接口的参数、返回值和枚举取值。

## 执行流程

1. **确保 SDK 为最新版本**：`pip install -U lebai_sdk`，用 `python3` 执行脚本（需 Python 3.7+，本地或板端均可）。
2. **读类型声明**：执行 `python3 -c "import lebai_sdk, os; print(os.path.dirname(lebai_sdk.__file__))"` 获取包安装路径，然后读取该目录下的 `__init__.pyi`，它是 Python 接口的唯一权威来源（类型 + 中文说明文档），接口签名、TypedDict 字段、Literal 枚举值一律以它为准，不要凭记忆猜测。
3. **设备发现（IP 未知时）**：用户要求查找设备时，调用 `lebai_sdk.discover_devices(time)` 扫描局域网，返回列表含 `name`/`mac`/`ip`/`online`，展示给用户让其选择。
4. **连接机器人**：

   ```python
   import lebai_sdk
   lebai = lebai_sdk.connect("192.168.2.101")            # 真实机器人，默认 3031 端口
   lebai = lebai_sdk.connect("192.168.2.101", True)      # 模拟器，3030 端口
   lebai = lebai_sdk.connect("192.168.2.101", 3032)      # 自定义端口
   ```

5. **开机流程**：`start_sys()` 启动系统，再 `poweron()` 上电，之后才能运动。
6. **运动控制**：`movej` / `movel` 参数为 `(p, a, v, t, r)`，`p` 传关节位姿 `list[float]`（6 个弧度）或笛卡尔位姿 `dict`（`x/y/z/rx/ry/rz`）；`t` 指定后忽略 `a/v`；返回值是运动 ID，可用 `wait_move(id)` 等待完成。圆弧用 `movec(via, p, rad, a, v, t, r)`。笛卡尔运动（`movel`/`movec`）前建议先反解出目标关节位姿，调用 `measure_manipulation` 判断当前位置和目标位置的灵活性，避免奇异位姿：

   ```python
   target_cart = {"x": 0.3, "y": 0.0, "z": 0.4, "rx": 3.14, "ry": 0.0, "rz": 0.0}
   current_joint = lebai.get_kin_data()["actual_joint_pose"]
   target_joint = lebai.kinematics_inverse(target_cart, current_joint)
   if lebai.measure_manipulation(current_joint) > 0.0001 and lebai.measure_manipulation(target_joint) > 0.0001:
       lebai.movel(target_cart, 0.5, 0.2)
   else:
       print("当前位置或目标位姿可达性低，请调整")
   ```
7. **状态查询**：`get_robot_state()`（如 `"IDLE"`、`"MOVING"`）、`get_kin_data()`（位置/速度/加速度/力矩）、`get_phy_data()`；运动前可用 `can_move(get_robot_state())` 判断机器人是否可运动，避免在 `"ESTOP"`/`"ROBOT_OFF"` 状态下直接发运动指令。
8. **异常处理**：所有接口为同步阻塞调用，失败抛出 `TypeError`，错误信息是字符串，直接展示给用户即可。

## 功能列表

### 任务管理

- `start_task(scene, params, dir, is_parallel, loop_to)` → 启动场景任务，返回任务 ID
- `wait_task(id)` → 阻塞等待任务完成，返回状态字符串
- `cancel_task(id)` / `pause_task(id)` / `resume_task(id)` → 控制任务生命周期
- `get_task_state(id)` / `get_task_list()` / `get_main_task_id()` → 查询任务状态

### 夹爪操作

- `init_claw(force)` → 初始化夹爪（`force=True` 强制初始化）
- `set_claw(force, amplitude)` → 设置力度和开度，只设置指定项
- `get_claw()` → 读取状态，返回 `{force, amplitude, weight, hold_on}`

### IO 与信号量

- 数字量：`set_do(device, pin, value)` / `get_di(device, pin)` → 读写数字输出/输入
- 模拟量：`set_ao(device, pin, value)` / `get_ai(device, pin)` → 读写模拟输出/输入（0~10V）
- 设备名：`"ROBOT"`、`"FLANGE"`、`"EXTRA"`、`"SHOULDER"`、`"FLANGE_BTN"`
- 信号量：`set_signal(index, value)` / `get_signal(index)` / `wait_signal(index, value, relation)`

### 订阅推送

- `sub = lebai.subscribe(method, param)` → 订阅 RPC 推送
- `sub.next()` → 阻塞获取下一条推送（JSON 字符串），返回 `None` 表示结束

### 安全相关

- `set_collision_detector_sensitivity(sensitivity)` → 设置碰撞检测灵敏度
- `enable_collision_detector()` / `disable_collision_detector()` → 开关碰撞检测
- `enable_joint_limits()` / `disable_joint_limits()` → 开关关节限位

## 常见问题

- **枚举全是字符串**：机器人状态 `"IDLE"`/`"MOVING"`/`"ESTOP"`，数字 IO 模式 `"INPUT"`/`"OUTPUT"`，串口校验位 `"None"`/`"Odd"`/`"Even"`（`"None"` 表示无校验），信号比较关系 `"EQ"`/`"NEQ"`/`"LT"`/`"LTE"`/`"GT"`/`"GTE"`，自动配置项 `"ARM_POWER"`/`"ENABLE_JOINT"`/`"INIT_CLAW"`。
- **位姿格式**：关节位姿是 6 元素列表（弧度），不是字典；笛卡尔位姿必须含 `x/y/z`（`rx/ry/rz` 可省略，默认 0）。
- **运动参数单位**：`movej`/`speedj` 的 `a`/`v` 是角量（rad/s²、rad/s），`movel`/`movec`/`speedl` 是线量（m/s²、m/s），不要混用。
- **`t` 和 `a/v` 二选一**：指定时间 `t`（秒）后加速度和速度参数会被忽略。
- **危险操作**：涉及真实机器人运动的任务脚本，先建议用 `connect(ip, True)` 模拟器验证，再切换真实机器人；运动相关脚本执行前必须经用户确认。
- **接口不存在或参数错误**：先检查 SDK 版本是否过旧，`pip install -U lebai_sdk` 升级后再试。
