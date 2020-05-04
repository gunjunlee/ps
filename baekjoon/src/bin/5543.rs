macro_rules! parse_line { ($($t: ty),+) => ({
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let mut iter = line.split_whitespace();
    ($(iter.next().unwrap().parse::<$t>().unwrap()),+)
})}

use std::cmp::min;

fn main(){
    let a = parse_line!(u32);
    let b = parse_line!(u32);
    let c = parse_line!(u32);
    let d = parse_line!(u32);
    let e = parse_line!(u32);
    println!("{}", min(a, min(b, c)) + min(d, e) - 50);
}