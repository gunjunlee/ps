n = 200000

cnt = [0 for i in range(n)]
prime = []
for i in range(2, n):
    if cnt[i] == 0:
        for j in range(i, n, i):
            cnt[j] = 1
        prime.append(i)

print(len(prime))
print(prime[10000])