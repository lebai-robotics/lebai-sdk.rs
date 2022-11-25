import lebai_sdk
lebai = lebai_sdk.connect("192.168.2.1", True)

# start_sys
lebai.call("start_sys", {})

# get_speed_factor
kin_factor = lebai.call("get_kin_factor", {})
print(kin_factor)
print(kin_factor['speed_factor'])

# load_pose
lebai.call("save_pose", {
    "name": "home",
    "data": {"joint":{
        "base":{
            "kind":"CURRENT"
        },
        "delta":{"joint":[-0.1,-0.1,-0.1,-0.1,-0.1,-0.1]},
    }},
})
p_home = lebai.call("load_pose", {"name": "home"})
print(p_home)

# move_joint
lebai.call("move_joint", {
    "pose": p_home,
    "param":{"velocity":0.2}
})

# sub_robot_state
sub_robot_state = lebai.subscribe("robot_state", {
    "interval_min":20,
    "interval_max":10000
})
data = sub_robot_state.next()
while data != None:
    print(data)
    data = sub_robot_state.next()
