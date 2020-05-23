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

fn main() {
    let (n, m) = parse_line!(usize, i64);
    let mut memories: Vec<i64> = parse_list!(i64);
    let mut costs: Vec<i64> = parse_list!(i64);

    let mut dp = vec![vec![0i64; 10001]; n + 1];
    for i in 0..n {
        for cost in 0..10001usize {
            if i == 0 {
                if costs[i] <= cost as i64 {
                    dp[i][cost] = memories[i];
                }
            } else {
                dp[i][cost] = dp[i - 1][cost];
                if cost as i64 >= costs[i] {
                    dp[i][cost] = std::cmp::max(
                        dp[i - 1][cost - costs[i] as usize] + memories[i],
                        dp[i][cost],
                    );
                }
            }
        }
    }
    for i in 0..10001 {
        if dp[n - 1][i] >= m {
            println!("{}", i);
            break;
        }
    }
}
