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
    let n = parse_line!(usize);
    let nums = parse_list!(i32);
    let nums_rev: Vec<i32> = nums.clone().into_iter().rev().collect();

    let mut dp = vec![1; n];
    let mut dp_rev = vec![1; n];

    for i in 0..n {
        for j in (0..i).rev() {
            if nums[i] > nums[j] {
                dp[i] = std::cmp::max(dp[j] + 1, dp[i]);
            }
            if nums_rev[i] > nums_rev[j] {
                dp_rev[i] = std::cmp::max(dp_rev[j] + 1, dp_rev[i]);
            }
        }
    }

    let mut max = 1;
    for i in 0..n {
        max = std::cmp::max(max, dp[i] + dp_rev[n - i - 1]);
    }
    println!("{}", max - 1);
}
