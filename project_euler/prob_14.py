from tqdm import tqdm

NUM = 100_0000 + 1

arr = {}
arr[1] = 1

def sett(i, arr):
    past = []
    while True:
        if arr.get(i):
            cnt = arr[i]
            break
        past.append(i)
        if i % 2 == 0:
            i = i // 2
        else:
            i = 3 * i + 1
    for i in past[::-1]:
        cnt += 1
        arr[i] = cnt

for i in tqdm(range(1, NUM)):
    sett(i, arr)

arr = sorted(list(arr.items()), key=lambda x: x[1])
print("answer:", arr[-1][0])
