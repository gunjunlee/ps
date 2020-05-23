macro_rules! parse_line { ($($t: ty),+) => ({
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let mut iter = line.split_whitespace();
    ($(iter.next().unwrap().parse::<$t>().unwrap()),+)
})}

fn main() {
    let n = parse_line!(usize);
    let mut arr_sizes: Vec<(i64, i64)> = Vec::new();
    for _ in 0..n {
        arr_sizes.push(parse_line!(i64, i64));
    }
    let mut dp = vec![vec![0; n]; n];
    let mut p = vec![vec![0; n]; n];
    for i in 1..n {
        for j in 0..n - i {
            dp[j][j + i] = dp[j + 1][j + i] + arr_sizes[j].0 * arr_sizes[j].1 * arr_sizes[j + i].1;
            for k in j..j + i {
                let mut flops = dp[j][k] + dp[k + 1][j + i];
                flops += arr_sizes[j].0 * arr_sizes[k].1 * arr_sizes[j + i].1;
                if flops < dp[j][j + i] {
                    dp[j][j + i] = flops;
                    p[j][i + j] = k;
                }
                dp[j][j + i] = std::cmp::min(flops, dp[j][j + i]);
            }
        }
    }
    println!("{}", dp[0][n - 1]);
    for i in p.iter() {
        for j in i.iter() {
            print!("{} ", j);
        }
        println!("");
    }
}
