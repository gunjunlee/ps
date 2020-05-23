macro_rules! parse_line { ($($t: ty),+) => ({
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let mut iter = line.split_whitespace();
    ($(iter.next().unwrap().parse::<$t>().unwrap()),+)
})}

use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::io::Write;

fn main() {
    let stdout = std::io::stdout();
    let mut out = std::io::BufWriter::new(stdout.lock());

    let mut pq: BinaryHeap<i32> = BinaryHeap::new();
    let mut rpq: BinaryHeap<Reverse<i32>> = BinaryHeap::new();

    let n = parse_line!(usize);
    for _ in 0..n {
        let m = parse_line!(i32);
        if pq.is_empty() {
            pq.push(m);
        } else {
            if &m <= pq.peek().unwrap() {
                pq.push(m);
                if pq.len() > rpq.len() + 1 {
                    rpq.push(Reverse(pq.pop().unwrap()));
                }
            } else {
                rpq.push(Reverse(m));
                if pq.len() < rpq.len() {
                    pq.push(rpq.pop().unwrap().0);
                }
            }
        }
        write!(out, "{}\n", pq.peek().unwrap());
    }
}
