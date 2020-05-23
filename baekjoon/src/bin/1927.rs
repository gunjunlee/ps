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

    let mut pq: BinaryHeap<Reverse<i32>> = BinaryHeap::new();
    let n = parse_line!(usize);
    for _ in 0..n {
        let m = parse_line!(i32);
        if m == 0 {
            match pq.pop() {
                Some(Reverse(i)) => write!(out, "{}\n", i).unwrap(),
                None => write!(out, "0\n").unwrap(),
            }
        } else {
            pq.push(Reverse(m));
        }
    }
}
