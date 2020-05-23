macro_rules! parse_line { ($($t: ty),+) => ({
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let mut iter = line.split_whitespace();
    ($(iter.next().unwrap().parse::<$t>().unwrap()),+)
})}

fn main() {
    let (a, mut b, c) = parse_line!(i64, i64, i64);
    let mut ans = 1;
    let mut apow = a;
    while b > 0 {
        if b % 2 == 1 {
            ans = (ans * apow) % c;
        }
        apow = (apow * apow) % c;
        b = b / 2;
    }
    println!("{}", ans);
}
