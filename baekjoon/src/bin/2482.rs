macro_rules! parse_line { ($($t: ty),+) => ({
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let mut iter = line.split_whitespace();
    ($(iter.next().unwrap().parse::<$t>().unwrap()),+)
})}

const MOD: i64 = 1_000_000_003;

fn main() {
    let n = parse_line!(usize);
    let k = parse_line!(usize);

    let mut dp: Vec<Vec<i64>> = vec![vec![0; k + 1]; n + 1];

    for i in 0..n + 1 {
        dp[i][1] = i as i64;
        dp[i][0] = 1;
    }

    for i in 2..n + 1 {
        for j in 2..k + 1 {
            dp[i][j] = (dp[i - 2][j - 1] + dp[i - 1][j]) % MOD;
        }
    }
    let ans = (dp[n - 3][k - 1] + dp[n - 1][k]) % MOD;
    println!("{}", ans);
}
