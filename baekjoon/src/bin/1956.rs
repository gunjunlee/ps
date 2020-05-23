macro_rules! parse_line { ($($t: ty),+) => ({
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let mut iter = line.split_whitespace();
    ($(iter.next().unwrap().parse::<$t>().unwrap()),+)
})}

use std::io::Write;

const MAX: i32 = i32::max_value();

fn main() {
    let stdout = std::io::stdout();
    let mut out = std::io::BufWriter::new(stdout.lock());

    let (n, m) = parse_line!(usize, usize);

    let mut dist: Vec<Vec<i32>> = vec![vec![MAX; n + 1]; n + 1];
    for _ in 0..m {
        let (e1, e2, w) = parse_line!(usize, usize, i32);
        dist[e1][e2] = std::cmp::min(w, dist[e1][e2]);
    }

    for k in 1..n + 1 {
        for i in 1..n + 1 {
            for j in 1..n + 1 {
                if dist[i][k] != MAX && dist[k][j] != MAX && dist[i][j] > dist[i][k] + dist[k][j] {
                    dist[i][j] = dist[i][k] + dist[k][j];
                }
            }
        }
    }
    let mut min = MAX;
    for i in 1..n + 1 {
        min = std::cmp::min(min, dist[i][i]);
    }
    println!("{}", if min != MAX { min } else { -1 });
}
