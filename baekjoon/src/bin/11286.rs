macro_rules! parse_line { ($($t: ty),+) => ({
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let mut iter = line.split_whitespace();
    ($(iter.next().unwrap().parse::<$t>().unwrap()),+)
})}

use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::io::Write;

#[derive(Debug)]
struct AbsNum(pub i32);

impl Ord for AbsNum {
    fn cmp(&self, other: &Self) -> Ordering {
        return match self.0.abs().cmp(&other.0.abs()) {
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater,
            Ordering::Equal => self.0.cmp(&other.0),
        };
    }
}

impl PartialOrd for AbsNum {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for AbsNum {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for AbsNum {}

fn main() {
    let stdout = std::io::stdout();
    let mut out = std::io::BufWriter::new(stdout.lock());

    let mut pq: BinaryHeap<Reverse<AbsNum>> = BinaryHeap::new();
    let n = parse_line!(usize);
    for _ in 0..n {
        let m = parse_line!(i32);
        if m == 0 {
            match pq.pop() {
                Some(Reverse(AbsNum(i))) => write!(out, "{}\n", i).unwrap(),
                None => write!(out, "0\n").unwrap(),
            }
        } else {
            pq.push(Reverse(AbsNum(m)));
        }
    }
}
