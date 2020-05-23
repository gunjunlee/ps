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
    from: usize,
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
    for _ in 0..parse_line!(usize) {
        let (n, m, t) = parse_line!(usize, usize, usize);
        let (s, g, h) = parse_line!(usize, usize, usize);
        let mut adj: Vec<Vec<(usize, i32)>> = vec![vec![(0, 0); 0]; n + 1];
        for _ in 0..m {
            let (p1, p2, w) = parse_line!(usize, usize, i32);
            adj[p1].push((p2, w));
            adj[p2].push((p1, w));
        }

        let mut pq: BinaryHeap<State> = BinaryHeap::new();
        let mut dists: Vec<i32> = vec![-1; n + 1];
        let mut froms: Vec<Vec<usize>> = vec![vec![0; 0]; n + 1];
        pq.push(State {
            v: s,
            dist: 0,
            from: 0,
        });
        loop {
            let state = match pq.pop() {
                Some(i) => i,
                None => break,
            };
            if dists[state.v] != -1 && dists[state.v] < state.dist {
                continue;
            }
            if dists[state.v] == state.dist {
                froms[state.v].push(state.from);
                continue;
            }
            dists[state.v] = state.dist;
            froms[state.v].push(state.from);
            for &(next_v, next_w) in adj[state.v].iter() {
                pq.push(State {
                    v: next_v,
                    dist: dists[state.v] + next_w,
                    from: state.v,
                });
            }
        }
        let mut ans: Vec<bool> = vec![false; n + 1];
        let mut targets: Vec<usize> = Vec::new();
        for _ in 0..t {
            let mut target = parse_line!(usize);
            targets.push(target);
            if ans[target] {
                continue;
            }
            let mut is_included = false;
            let mut routes: Vec<usize> = Vec::new();
            let mut considered: Vec<usize> = Vec::new();
            routes.push(target);
            while !routes.is_empty() {
                let route = routes.pop().unwrap();
                if route == g && froms[route].contains(&h) {
                    is_included = true;
                }
                if route == h && froms[route].contains(&g) {
                    is_included = true;
                }
                for &from in froms[route].iter() {
                    if !considered.contains(&from) {
                        routes.push(from);
                        considered.push(from);
                    }
                }
            }
            if is_included {
                ans[target] = true;
            }
        }
        targets.sort();
        for &target in targets.iter() {
            if ans[target] {
                print!("{} ", target);
            }
        }
        println!();
    }
}

// 1
// 5 4 2
// 1 2 3
// 1 2 6
// 2 3 2
// 3 4 4
// 3 5 3
// 5
// 4
