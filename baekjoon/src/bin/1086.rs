macro_rules! parse_line { ($($t: ty),+) => ({
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let mut iter = line.split_whitespace();
    ($(iter.next().unwrap().parse::<$t>().unwrap()),+)
})}

fn poo(
    mut dp: &mut Vec<Vec<i64>>,
    remainder: &Vec<(usize, usize)>,
    k: usize,
    stat: usize,
    N: usize,
    K: usize,
) -> i64 {
    if dp[k][stat] == -1 {
        if stat == 0 {
            dp[0][stat] = 1;
            for i in 1..K {
                dp[i][stat] = 0;
            }
        } else {
            for i in 0..K {
                dp[i][stat] = 0;
            }
            for i in 0..N {
                if (1 << i) & stat != 0 {
                    for j in 0..K {
                        // j: remainder
                        let (a, b) = remainder[i];
                        let (a, b) = (a as usize, b as usize);
                        dp[(j * b + a) % K][stat] +=
                            poo(&mut dp, &remainder, j, stat - (1 << i), N, K);
                    }
                }
            }
        }
    }
    return dp[k][stat];
}

fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        return a;
    } else if b > a {
        return gcd(b, a);
    } else {
        return gcd(b, a % b);
    }
}

fn get_remainder(nums: &String, k: usize) -> (usize, usize) {
    let mut a = 0;
    let mut b = 1;
    for i in (0..nums.len()).rev() {
        let n: usize = nums.chars().nth(i).unwrap().to_digit(10).unwrap() as usize;
        a = (a + (n * b)) % k;
        b = b * 10 % k;
    }
    return (a, b);
}

fn main() {
    let n = parse_line!(usize);
    let mut nums: Vec<String> = Vec::new();
    for _ in 0..n {
        nums.push(parse_line!(String).trim().to_string());
    }
    let k = parse_line!(usize);
    let mut dp = vec![vec![-1i64; 1 << n]; k];

    let mut remainder: Vec<(usize, usize)> = Vec::new();

    for i in 0..n {
        let num = &nums[i];
        remainder.push(get_remainder(&num, k));
    }
    // println!("{:?}", remainder);

    let ans = poo(&mut dp, &remainder, 0, (1 << n) - 1, n, k);
    let mut cnt: i64 = 1;
    for i in 1..n + 1 {
        cnt *= i as i64;
    }

    // print
    let gcd = gcd(ans, cnt);
    println!("{}/{}", ans / gcd, cnt / gcd);
}

// 5
// 1
// 3
// 630
// 233
// 4
// 10

// 15
// 31111111122222211111111111111111111222222222222
// 71111111122222211111111111111111111222222222222
// 21111111122222211111111111111111111222222222222
// 81111111122222211111111111111111111222222222222
// 91111111122222211111111111111111111222222222222
// 21111111122222211111111111111111111222222222222
// 41111111122222211111111111111111111222222222222
// 51111111122222211111111111111111111222222222222
// 81111111122222211111111111111111111222222222222
// 31111111122222211111111111111111111222222222222
// 51111111122222211111111111111111111222222222222
// 41111111122222211111111111111111111222222222222
// 81111111122222211111111111111111111222222222222
// 41111111122222211111111111111111111222222222222
// 31111111122222211111111111111111111222222222222
// 100

// 5
// 5221
// 40
// 1
// 58
// 9
// 3
