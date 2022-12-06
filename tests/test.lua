local lebai_sdk = require('lebai_sdk')

-- 设备发现
print(lebai_sdk.discover_devices(2))

local lebai = lebai_sdk.connect("127.0.0.1", true)

-- start_sys
lebai:call("start_sys", "{}")

jPose_1 = {0,-0.7854,1.57,-0.7854,1.57,0}
cPose_1 = {x=-0.565,y=-0.12,z=0.13,rz=-1.57,ry=0,rx=1.57}
jPose_2 = {0,-0.566,1.12,-0.55,1.57,0}
cPose_2 = {x=-0.64,y=-0.12,z=0.13,rz=-1.57,ry=0,rx=1.5}

lebai:movej(jPose_1, 0, 0, 3)
lebai:movel(cPose_2, 0, 0, 3)

lebai:movej(cPose_1, 0, 0, 3)
lebai:movel(jPose_2, 0, 0, 3)

print(lebai:kinematics_forward(jPose_1))
print(lebai:kinematics_inverse(cPose_1))
print(lebai:kinematics_inverse(jPose_2))
print(lebai:kinematics_forward(cPose_2))
