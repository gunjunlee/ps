macro_rules! parse_line { ($($t: ty),+) => ({
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let mut iter = line.split_whitespace();
    ($(iter.next().unwrap().parse::<$t>().unwrap()),+)
})}

fn main() {
    let n = parse_line!(i64);
    let k = parse_line!(i64);

    let mut left = 0;
    let mut right = n * n;
    let mut result = 0;
    while left <= right {
        let mut cnt = 0;
        let mid = (left + right) / 2;

        for i in 1..n + 1 {
            cnt += std::cmp::min(mid / i, n);
        }
        if cnt < k {
            left = mid + 1;
        } else {
            result = mid;
            right = mid - 1;
        }
    }
    println!("{}", result);
}
