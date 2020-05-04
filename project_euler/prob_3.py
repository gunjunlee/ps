def is_3digit(n):
    return (n >= 100 and n < 1000)

n = 999

# 11 is gcd(100001, 10010, 1100)

while True:
    n_ = n * 1000 + int(n/100) + int(n%100 /10)*10 + n%10*100
    n = n - 1
    for i in range(110, 1000, 11):
        if n_ % i == 0:
            if is_3digit(n_/i):
                print(n_)
                exit(0)

