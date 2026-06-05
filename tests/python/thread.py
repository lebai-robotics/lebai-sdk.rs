import threading
import time
import lebai_sdk

print(lebai_sdk.version())
lebai = lebai_sdk.connect("47.116.126.158", True)

def get_kin_data(i):
    while True:
        kin_data = lebai.get_kin_data()['actual_joint_pose'][0]
        print(i, kin_data)

if __name__ == '__main__':
    threads = []
    for i in range(10):
        t = threading.Thread(target=get_kin_data, args=(i,))
        threads.append(t)
        t.start()

    for i in range(10):
        threads[i].join()
