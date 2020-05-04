from tqdm import tqdm
n = 2000001

cnt = [0 for i in range(n)]
prime = []
for i in tqdm(range(2, n)):
    if cnt[i] == 0:
        for j in range(i, n, i):
            cnt[j] = 1
        prime.append(i)
# print(prime)
print(sum(prime))