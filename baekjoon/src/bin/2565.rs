macro_rules! parse_line { ($($t: ty),+) => ({
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let mut iter = line.split_whitespace();
    ($(iter.next().unwrap().parse::<$t>().unwrap()),+)
})}

fn main() {
    let n = parse_line!(usize);
    let mut lines = Vec::<(usize, usize)>::new();
    for _ in 0..n {
        lines.push(parse_line!(usize, usize));
    }
    lines.sort_by(|(x0, _y0), (x1, _y1)| x0.cmp(&x1));

    let nums: Vec<usize> = lines.into_iter().map(|(_, x)| x).collect();

    let mut dp = vec![1; n];
    let mut dp_rev = vec![1; n];

    for i in 0..n {
        for j in (0..i).rev() {
            if nums[i] > nums[j] {
                dp[i] = std::cmp::max(dp[j] + 1, dp[i]);
            }
        }
    }

    for i in (0..n).rev() {
        for j in (i+1..n).rev() {
            if nums[i] < nums[j] {
                dp_rev[i] = std::cmp::max(dp_rev[j] + 1, dp_rev[i]);
            }
        }
    }

    let mut max = 1;
    for i in 0..n {
        max = std::cmp::max(max, dp[i] + dp_rev[i]);
    }
    println!("{}", n - (max - 1));
}

// 6
// 1 8
// 2 9
// 4 4
// 5 2
// 6 1
// 7 5