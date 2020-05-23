macro_rules! parse_line { ($($t: ty),+) => ({
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let mut iter = line.split_whitespace();
    ($(iter.next().unwrap().parse::<$t>().unwrap()),+)
})}

fn dfs(
    start: usize,
    edges: &Vec<Vec<(usize, i32, i32)>>,
    mut visited: &mut Vec<bool>,
    distance: i32,
    cost: i32,
    budget: i32,
    target: usize,
) -> (i32, i32) {
    if start == target {
        return (distance, cost);
    }
    let (mut min_dist, mut min_cost) = (i32::max_value(), i32::max_value());
    visited[start] = true;
    for edge in edges[start].iter() {
        let e = edge.0;
        if !visited[e] {
            if budget >= cost + edge.1 {
                let (dist, c) = dfs(
                    e,
                    &edges,
                    &mut visited,
                    distance + edge.2,
                    cost + edge.1,
                    budget,
                    target,
                );
                if c <= budget && dist < min_dist {
                    min_dist = dist;
                    min_cost = c;
                }
            }
        }
    }
    visited[start] = false;
    return (min_dist, min_cost);
}

fn main() {
    let T = parse_line!(usize);
    for _ in 0..T {
        let (n, m, k) = parse_line!(usize, i32, usize);
        let mut edges: Vec<Vec<(usize, i32, i32)>> = vec![vec![(0, 0, 0); 0]; n + 1];
        for _ in 0..k {
            let (u, v, c, d) = parse_line!(usize, usize, i32, i32);
            edges[u].push((v, c, d));
        }

        let mut visited: Vec<bool> = vec![false; n + 1];
        let (min_dist, min_cost) = dfs(1, &edges, &mut visited, 0, 0, m, n);

        if min_cost != i32::max_value() {
            println!("{}", min_dist);
        } else {
            println!("Poor KCM");
        }
    }
}
