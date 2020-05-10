macro_rules! parse_line { ($($t: ty),+) => ({
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let mut iter = line.split_whitespace();
    ($(iter.next().unwrap().parse::<$t>().unwrap()),+)
})}

fn count_dive(n: u64, d: u64) -> u64 {
    let mut divs = 0;
    let mut div = d;
    while n / div > 0 {
        divs += n / div;
        div *= d;
    }
    return divs;
}

fn main() {
    let (n, m) = parse_line!(u64, u64);
    let fives_n = count_dive(n, 5);
    let fives_m = count_dive(m, 5);
    let fives_n_m = count_dive(n-m, 5);
    let twos_n = count_dive(n, 2);
    let twos_m = count_dive(m, 2);
    let twos_n_m = count_dive(n-m, 2);
    println!("{}", std::cmp::min(fives_n - fives_m - fives_n_m, twos_n-twos_m-twos_n_m));
}