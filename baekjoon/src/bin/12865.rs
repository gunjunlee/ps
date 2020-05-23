macro_rules! parse_line { ($($t: ty),+) => ({
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let mut iter = line.split_whitespace();
    ($(iter.next().unwrap().parse::<$t>().unwrap()),+)
})}

fn main() {
    let (n, k) = parse_line!(usize, i32);
    let mut goods = Vec::<(i32, i32)>::new();
    let mut dp = vec![vec![0; n + 1]; k as usize + 1];
    goods.push((0, 0));
    for _ in 0..n {
        goods.push(parse_line!(i32, i32));
    }
    for i in 1..n + 1 {
        for j in 0..(k + 1) as usize {
            dp[j][i] = dp[j][i - 1];
            if goods[i].0 <= j as i32 {
                dp[j][i] = std::cmp::max(dp[j][i], dp[j - goods[i].0 as usize][i - 1] + goods[i].1);
            }
        }
    }
    println!("{}", dp[k as usize][n]);
}
