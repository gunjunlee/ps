macro_rules! parse_line { ($($t: ty),+) => ({
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let mut iter = line.split_whitespace();
    ($(iter.next().unwrap().parse::<$t>().unwrap()),+)
})}

use std::cmp::max;

fn main(){
    loop {
        let (a, b, c) = parse_line!(i32, i32, i32);
        if a | b | c == 0 { break; }
        let m = max(a, max(b, c));
        println!("{}", if -2 * m*m + a*a + b*b + c*c == 0 {"right"} else {"wrong"});
    }
}