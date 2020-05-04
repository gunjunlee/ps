from math import ceil

# fibonacci

TARGET = 23416728348467685

a, b = 1, 1
p, n = 0, 0

while True:
    a, b = b, a + b

    p, n = n, n + p + a

    if b >= TARGET:
        answer = b + n
        break
print(answer)
