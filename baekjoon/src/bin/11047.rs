macro_rules! parse_line { ($($t: ty),+) => ({
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let mut iter = line.split_whitespace();
    ($(iter.next().unwrap().parse::<$t>().unwrap()),+)
})}

fn main() {
    let (mut n, mut k) = parse_line!(usize, i32);
    let mut coins = vec![0; n];
    for i in 0..n {
        coins[i] = parse_line!(i32);
    }
    n -= 1;
    let mut ans = 0;
    while k > 0 {
        if k >= coins[n] {
            ans += 1;
            k -= coins[n];
        } else {
            n -= 1;
        }
    }
    println!("{}", ans);
}
