import math

class PrimeCalculator:
    def __init__(self):
        self.primes = [2]
        self.max_num = 2

    def get_prime_factors(self, number):
        self.set_primes(number)

        cnt_primes = {}
        for prime in self.primes:
            cnt = 0
            while number % prime == 0:
                number = int(number / prime)
                cnt += 1
            if cnt > 0:
                cnt_primes[prime] = cnt
        return cnt_primes

    def get_number_of_divisors(self, number):
        prime_factors = self.get_prime_factors(number)

        return self.get_number_of_divisors_from_prime_factors(prime_factors)

    @staticmethod
    def get_number_of_divisors_from_prime_factors(prime_factors):
        num = 1
        for cnt in prime_factors.values():
            num *= cnt + 1

        return num

    def set_primes(self, number):
        for i in range(self.max_num + 1, number+1):
            self._set_prime(i)
        self.max_num = number

    def _set_prime(self, number):
        # only called from selt_primes
        # TODO: move this function into set_primes

        if self.max_num >= number:
            return

        for prime in self.primes:
            if prime * prime > number:
                self.primes.append(number)
                break
            if number % prime == 0:
                break
        return

    @staticmethod
    def check_prime(target, primes):
        # `primes` are list of all primes lower than `target`
        for prime in target:
            if target % prime == 0:
                return False
        return True



prime_calculator = PrimeCalculator()

max_divisor = 3

for i in range(1, 10000000):
    prime_factors_1 = prime_calculator.get_prime_factors(i)
    prime_factors_2 = prime_calculator.get_prime_factors(i+1)

    for prime, cnt in  prime_factors_2.items():
        prime_factors_1[prime] = prime_factors_1.get(prime, 0) + cnt
    prime_factors_1[2] = prime_factors_1[2] - 1

    num_divisors = prime_calculator.get_number_of_divisors_from_prime_factors(prime_factors_1)

    if num_divisors > max_divisor:
        print(num_divisors, i * (i+1) // 2)
        max_divisor = num_divisors
        if num_divisors >= 500:
            break

print("answer is: ", i * (i+1) // 2)