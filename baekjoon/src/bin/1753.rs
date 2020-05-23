macro_rules! parse_line { ($($t: ty),+) => ({
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let mut iter = line.split_whitespace();
    ($(iter.next().unwrap().parse::<$t>().unwrap()),+)
})}

use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::io::Write;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct State {
    v: usize,
    dist: i32,
}

impl Ord for State {
    fn cmp(&self, other: &State) -> Ordering {
        other.dist.cmp(&self.dist)
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &State) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    let stdout = std::io::stdout();
    let mut out = std::io::BufWriter::new(stdout.lock());

    let (v, e) = parse_line!(usize, usize);
    let k = parse_line!(usize) - 1;
    let mut adj: Vec<Vec<(usize, i32)>> = vec![vec![(0, 0); 0]; v];
    for _ in 0..e {
        let (p1, p2, w) = parse_line!(usize, usize, i32);
        adj[p1 - 1].push((p2 - 1, w));
    }
    let mut dists: Vec<i32> = vec![-1; v];
    let mut pq: BinaryHeap<State> = BinaryHeap::new();
    pq.push(State { v: k, dist: 0 });
    while !pq.is_empty() {
        let state = pq.pop().unwrap();
        if dists[state.v] != -1 {
            continue;
        }
        dists[state.v] = state.dist;
        for &(next_v, next_w) in adj[state.v].iter() {
            pq.push(State {
                v: next_v,
                dist: dists[state.v] + next_w,
            });
        }
    }
    for &dist in dists.iter() {
        if dist == -1 {
            writeln!(out, "INF");
        } else {
            writeln!(out, "{}", dist);
        }
    }
}
