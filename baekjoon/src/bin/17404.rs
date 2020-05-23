macro_rules! parse_line { ($($t: ty),+) => ({
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let mut iter = line.split_whitespace();
    ($(iter.next().unwrap().parse::<$t>().unwrap()),+)
})}

use std::cmp;

fn main() {
    let n = parse_line!(usize);

    let mut dp: Vec<Vec<Vec<i32>>> = vec![vec![vec![i32::max_value(); 3]; 3]; n + 1];

    for i in 1..n + 1 {
        let (r, g, b) = parse_line!(i32, i32, i32);
        let colors = vec![r, g, b];
        if i == 1 {
            for j in 0..3 {
                dp[i][j][j] = colors[j];
            }
        } else {
            for j in 0..3 {
                for k in 0..3 {
                    let k_1 = (k + 1) % 3;
                    let k_2 = (k + 2) % 3;
                    let min = cmp::min(dp[i - 1][j][k_1], dp[i - 1][j][k_2]);
                    if min == i32::max_value() {
                        dp[i][j][k] = min;
                    } else {
                        dp[i][j][k] = min + colors[k];
                    }
                }
            }
        }
    }
    let mut ans = i32::max_value();
    for i in 0..3 {
        for j in 0..3 {
            if i != j {
                ans = cmp::min(ans, dp[n][i][j]);
            }
        }
    }
    println!("{}", ans);
}
// 6
// 10 20 30
// 10 20 30
// 10 20 30
// 10 20 30
// 10 20 30
// 10 20 30
