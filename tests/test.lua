local json = require("json")
local Lebai = require('lebai_sdk')
local lebai = Lebai.connect("127.0.0.1", true)

-- get_speed_factor
kin_factor = lebai:call("get_kin_factor")
print(kin_factor)
kin_factor = json.decode(kin_factor)
print(kin_factor)
print(kin_factor.speed_factor)

-- load_pose
pose = '{"joint":{"base":{"kind":"CURRENT"},"delta":{"joint":[0.1,0.1,0.1,0.1,0.1,0.1]}}}'
lebai:call("save_pose", '{"name":"home","data":'..pose..'}')
pose = lebai:call("load_pose", '{"name": "home"}')
print(pose)

-- move_joint
motion_id = lebai:call("move_joint", '{"pose":'..pose..',"param":{"time":3}}')
print(motion_id)
lebai:call("wait_move", motion_id)
lebai:call("move_joint", '{"pose":{"joint":{"base":{"kind":"CURRENT"},"delta":{"joint":[-0.1,-0.1,-0.1,-0.1,-0.1,-0.1]}}},"param":{"time":3}}')

-- sub_robot_state
sub_robot_state = lebai:subscribe("robot_state", '{"interval_min":100,"interval_max":10000}')
data = sub_robot_state:next()
while(data~=nil)
do
    print(data)
    data = sub_robot_state:next()
end
