# 40 C 20

def factorial(n):
    s = 1
    for i in range(1, n+1):
        s *= i
    return s

print(factorial(40) // factorial(20) // factorial(20))