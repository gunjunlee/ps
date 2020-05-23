macro_rules! parse_line { ($($t: ty),+) => ({
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let mut iter = line.split_whitespace();
    ($(iter.next().unwrap().parse::<$t>().unwrap()),+)
})}

use std::cmp::min;

fn main() {
    let (x, y, w, h) = parse_line!(i32, i32, i32, i32);
    println!("{}", min(min(x, w - x), min(y, h - y)));
}
