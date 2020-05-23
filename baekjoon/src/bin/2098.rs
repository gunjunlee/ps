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

const MAX: i32 = i32::max_value();

fn poo(
    mut dp: &mut Vec<Vec<i32>>,
    w: &Vec<Vec<i32>>,
    cur: usize,
    stat: usize,
    depth: usize,
    start: usize,
) -> i32 {
    if dp[cur][stat] == -1 {
        dp[cur][stat] = MAX;
        if depth == 0 && cur == start {
            dp[cur][stat] = 0;
        } else {
            for i in 0..dp.len() {
                if w[i][cur] != 0 && ((stat & (1 << i)) != 0 || depth == dp.len()) {
                    let weight_traj = poo(&mut dp, &w, i, stat ^ (1 << i), depth - 1, start);
                    if weight_traj != MAX {
                        dp[cur][stat] = std::cmp::min(weight_traj + w[i][cur], dp[cur][stat]);
                    }
                }
            }
        }
    }
    return dp[cur][stat];
}

fn main() {
    let n = parse_line!(usize);
    let mut w: Vec<Vec<i32>> = vec![vec![0; 0]; n];

    for i in 0..n {
        w[i] = parse_list!(i32);
    }
    let mut dp: Vec<Vec<i32>> = vec![vec![-1; 1 << n]; n]; // last point: n, trajectory : ex) 0b0000111100
    let mut min: i32 = poo(&mut dp, &w, 0, (1 << n) - 1, n, 0);
    println!("{}", min);
}

// 4
// 0 1 2 3
// 2 0 3 0
// 3 0 0 0
// 1 2 3 0

// 4
// 0 1 100 100
// 0 0 3 0
// 0 200 0 400
// 1 1 1 0
