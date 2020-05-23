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

use std::io::Write;

fn main() {
    let _n = parse_line!(usize);
    let mut nums = parse_list!(i32);
    nums.sort();
    let _m = parse_line!(usize);
    let queries = parse_list!(i32);

    let stdout = std::io::stdout();
    let mut out = std::io::BufWriter::new(stdout.lock());

    for q in queries.iter() {
        match nums.binary_search(&q) {
            Ok(_) => write!(out, "1\n"),
            Err(_) => write!(out, "0\n"),
        };
    }
}
