a = 2
b = 3
sum = 0

while a <= 4000000:
    print(a, b)
    if a%2 == 0: sum = sum + a
    temp = b
    b = a + b
    a = temp

print(sum)