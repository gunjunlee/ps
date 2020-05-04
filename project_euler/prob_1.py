def acc_sum(n):
    return (n+1)*n/2

n = 999
a = 3
b = 5

num_mul_a = int(n/a)
num_mul_b = int(n/b)
num_mul_ab = int(n/(a*b))

print(num_mul_a, num_mul_b, num_mul_ab)

res = acc_sum(num_mul_a)*a+acc_sum(num_mul_b)*b-acc_sum(num_mul_ab)*a*b

print(res)