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

    let n = parse_line!(usize);
    let m = parse_line!(usize);

    let mut dist: Vec<Vec<i32>> = vec![vec![MAX; n + 1]; n + 1];
    for _ in 0..m {
        let (e1, e2, w) = parse_line!(usize, usize, i32);
        dist[e1][e2] = std::cmp::min(w, dist[e1][e2]);
    }
    for i in 1..n + 1 {
        dist[i][i] = 0;
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
    for i in 1..n + 1 {
        for j in 1..n + 1 {
            write!(out, "{} ", if dist[i][j] != MAX { dist[i][j] } else { 0 });
        }
        writeln!(out);
    }
}
