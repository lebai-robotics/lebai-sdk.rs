---
name: lebai-robot
description: 使用 Python SDK 调用 Lebai 协作机器人（机械臂）
version: 1.0.0
trigger: lebai,乐白,机器人,机械臂,夹爪,运动,移动
risk: medium
permissions: device_exec
delegate_preference: hybrid
requires_board: false
approval_level: confirm
cooldown_seconds: 0
scheduler_template: none
category: Lebai机器人
---

# Lebai 机器人 Python SDK 使用

## 适用场景

- 需要编写或修改调用 Lebai 协作机器人 Python API 的代码（连接、运动控制、IO、信号量、任务、夹爪、MODBUS、串口、LED 等）。
- 需要解释已有 Python SDK 代码的含义，或排查调用报错。
- 需要确认某个接口的参数、返回值和枚举取值。

## 执行流程

1. **确保 SDK 为最新版本**：先 `pip install -U lebai_sdk` 升级到最新版，再调用 `lebai_sdk.version()` 确认版本；`lebai_sdk.pyi` 中新增的接口需要对应版本的 SDK 才支持，若运行时提示接口不存在（AttributeError）或报参数错误，先检查是否为版本过旧导致。
2. **读类型声明**：读取 `lebai_sdk.pyi`，它是 Python 接口的唯一权威来源（类型 + 中文说明文档），接口签名、TypedDict 字段、Literal 枚举值一律以它为准，不要凭记忆猜测。
3. **连接机器人**：

   ```python
   import lebai_sdk
   lebai = lebai_sdk.connect("192.168.2.101")            # 真实机器人，默认 3031 端口
   lebai = lebai_sdk.connect("192.168.2.101", True)      # 模拟器，3030 端口
   lebai = lebai_sdk.connect("192.168.2.101", 3032)      # 自定义端口
   ```

4. **开机流程**：`start_sys()`，然后才能运动。
5. **运动控制**：`movej` / `movel` 参数为 `(p, a, v, t, r)`，`p` 传关节位姿 `list[float]`（6 个弧度）或笛卡尔位姿 `dict`（`x/y/z/rx/ry/rz`）；`t` 指定后忽略 `a/v`；返回值是运动 ID，可用 `wait_move(id)` 等待完成。圆弧用 `movec(via, p, rad, a, v, t, r)`。
6. **状态查询**：`get_robot_state()`（如 `"IDLE"`、`"MOVING"`）、`get_kin_data()`（位置/速度/加速度/力矩）、`get_phy_data()`；运动前可用 `can_move(get_robot_state())` 判断机器人是否可运动，避免在 `"ESTOP"`/`"ROBOT_OFF"` 状态下直接发运动指令。
7. **IO 与信号量**：数字量 `set_do`/`get_di`，模拟量 `set_ao`/`get_ai`，设备名为大写字符串（`"ROBOT"`、`"FLANGE"`、`"EXTRA"`、`"SHOULDER"`、`"FLANGE_BTN"`）；信号量用 `set_signal`/`get_signal`/`wait_signal`。
8. **异常处理**：所有接口为同步阻塞调用，失败抛出 `TypeError`，错误信息是字符串，直接展示给用户即可。
9. **订阅推送**：`sub = lebai.subscribe(method, param)` 后循环 `sub.next()` 获取 JSON 字符串，返回 `None` 表示结束。

## 工具映射

| 用途 | 工具/命令 |
| --- | --- |
| 接口签名与文档 | 读取 `lebai_sdk.pyi`（类型声明与说明的唯一权威来源） |
| 编写/运行脚本 | 板端 `python3` 执行（需已安装 `lebai_sdk` Python 包） |
| 查看 SDK 版本 | `python3 -c "import lebai_sdk; print(lebai_sdk.version())"` |
| 模拟器验证 | `connect(ip, True)` 连接 3030 模拟器端口，可安全调试运动参数 |
| 设备发现 | `lebai_sdk.discover_devices(time)` 返回在线设备列表（含 name/mac/ip/online），用于查找机器人 IP |

## 常见问题

- **枚举全是字符串**：机器人状态 `"IDLE"`/`"MOVING"`/`"ESTOP"`，数字 IO 模式 `"INPUT"`/`"OUTPUT"`，串口校验位 `"None"`/`"Odd"`/`"Even"`（`"None"` 表示无校验），信号比较关系 `"EQ"`/`"NEQ"`/`"LT"`/`"LTE"`/`"GT"`/`"GTE"`，自动配置项 `"ARM_POWER"`/`"ENABLE_JOINT"`/`"INIT_CLAW"`。
- **位姿格式**：关节位姿是 6 元素列表（弧度），不是字典；笛卡尔位姿必须含 `x/y/z`（`rx/ry/rz` 可省略，默认 0）。
- **运动参数单位**：`movej`/`speedj` 的 `a`/`v` 是角量（rad/s²、rad/s），`movel`/`movec`/`speedl` 是线量（m/s²、m/s），不要混用。
- **`t` 和 `a/v` 二选一**：指定时间 `t`（秒）后加速度和速度参数会被忽略。
- **危险操作**：涉及真实机器人运动的任务脚本，先建议用 `connect(ip, True)` 模拟器验证，再切换真实机器人；运动相关脚本执行前必须经用户确认。
