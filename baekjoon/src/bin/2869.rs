macro_rules! parse_line { ($($t: ty),+) => ({
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let mut iter = line.split_whitespace();
    ($(iter.next().unwrap().parse::<$t>().unwrap()),+)
})}

fn main() {
    let (a, b, v) = parse_line!(i32, i32, i32);
    println!("{}", (v - b - 1) / (a - b) + 1);
}
