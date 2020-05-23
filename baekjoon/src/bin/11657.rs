macro_rules! parse_line { ($($t: ty),+) => ({
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let mut iter = line.split_whitespace();
    ($(iter.next().unwrap().parse::<$t>().unwrap()),+)
})}

fn main() {
    let (n, m) = parse_line!(usize, usize);
    let mut edges: Vec<Vec<i64>> = vec![vec![i64::max_value(); n + 1]; n + 1];
    let mut dists: Vec<i64> = vec![i64::max_value(); n + 1];
    for _ in 0..m {
        let (p0, p1, d) = parse_line!(usize, usize, i64);
        edges[p0][p1] = std::cmp::min(d, edges[p0][p1]);
    }
    let mut edges_list: Vec<(usize, usize, i64)> = Vec::new();
    for i in 1..n + 1 {
        for j in 1..n + 1 {
            let e = edges[i][j];
            if e != i64::max_value() {
                edges_list.push((i, j, e));
            }
        }
    }
    dists[1] = 0; // start
    for _ in 0..n - 1 {
        for edge in edges_list.iter() {
            if dists[edge.0] == i64::max_value() {
                continue;
            }
            if dists[edge.0] + edge.2 < dists[edge.1] {
                dists[edge.1] = dists[edge.0] + edge.2;
            }
        }
    }
    // println!("{:?}", edges_list);
    // println!("{:?}", dists);
    // check negative cycle
    for edge in edges_list.iter() {
        if dists[edge.0] != i64::max_value() {
            if dists[edge.0] + edge.2 < dists[edge.1] {
                println!("-1");
                return;
            }
        }
    }
    for i in 2..n + 1 {
        println!(
            "{}",
            if dists[i] == i64::max_value() {
                -1
            } else {
                dists[i]
            }
        );
    }
}
