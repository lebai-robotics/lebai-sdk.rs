import threading
import time
import lebai_sdk

print(lebai_sdk.version())
lebai = lebai_sdk.connect("192.168.2.101", True)

def get_kin_data(i):
    counter = 0
    while True:
        lebai.get_kin_data()
        counter += 1
        if counter % 100 == 0:
            print(f"{i}, count: {counter}")

if __name__ == '__main__':
    threads = []
    for i in range(10):
        t = threading.Thread(target=get_kin_data, args=(i,))
        threads.append(t)
        t.start()

    for i in range(10):
        threads[i].join()
