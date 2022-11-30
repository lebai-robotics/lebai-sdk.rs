import lebai_sdk

# 设备发现
print(lebai_sdk.discover_devices(10))

lebai = lebai_sdk.connect("192.168.2.1", True)

# start_sys
lebai.call("start_sys", "{}")

jPose_1 = {"j1":0,"j2":-0.7854,"j3":1.57,"j4":-0.7854,"j5":1.57,"j6":0}
cPose_1 = [-0.565,-0.12,0.13,-1.57,0,1.57]
jPose_2 = {"j1":0,"j2":-0.566,"j3":1.12,"j4":-0.55,"j5":1.57,"j6":0}
cPose_2 = [-0.64,-0.12,0.13,-1.57,0,1.5]

lebai.movej(jPose_1, 0, 0, 3)
lebai.movel(cPose_2, 0, 0, 3)

lebai.movej(cPose_1, 0, 0, 3)
lebai.movel(jPose_2, 0, 0, 3)

print(lebai.kinematics_forward(jPose_1))
print(lebai.kinematics_inverse(cPose_1))
print(lebai.kinematics_inverse(jPose_2))
print(lebai.kinematics_forward(cPose_2))
