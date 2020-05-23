macro_rules! parse_line { ($($t: ty),+) => ({
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let mut iter = line.split_whitespace();
    ($(iter.next().unwrap().parse::<$t>().unwrap()),+)
})}

macro_rules! parse_list {
    ($t: ty) => {{
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        let list: Vec<$t> = line
            .split_whitespace()
            .map(|w| w.parse::<$t>().unwrap())
            .collect();
        list
    }};
}

use std::io::Write;

fn main() {
    let stdout = std::io::stdout();
    let mut out = std::io::BufWriter::new(stdout.lock());

    let n = parse_line!(usize);
    let nums = parse_list!(i32);
    let m = parse_line!(usize);

    let mut dp = vec![vec![false; n]; n];

    for i in 0..n {
        dp[i][i] = true;
    }

    for i in 1..n {
        for j in 0..n - i {
            if i == 1 {
                if nums[j] == nums[j + i] {
                    dp[j][j + i] = true;
                }
            } else {
                if dp[j + 1][j + i - 1] == true {
                    if nums[j] == nums[i + j] {
                        dp[j][j + i] = true;
                    }
                }
            }
        }
    }

    for _ in 0..m {
        let (s, e) = parse_line!(usize, usize);
        writeln!(out, "{}", if dp[s - 1][e - 1] { "1" } else { "0" }).unwrap();
    }
}
