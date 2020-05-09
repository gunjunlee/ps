macro_rules! parse_line { ($($t: ty),+) => ({
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let mut iter = line.split_whitespace();
    ($(iter.next().unwrap().parse::<$t>().unwrap()),+)
})}

fn main() {
    let n = parse_line!(i32);
    let mut schedules = Vec::<(i32, i32)>::new();
    for _ in 0..n {
        schedules.push(parse_line!(i32, i32));
    }
    schedules.sort_by(|(x0, y0), (x1, y1)| match x0.cmp(&x1) {
        std::cmp::Ordering::Equal => y0.cmp(&y1),
        _ => x0.cmp(&x1),
    });

    let mut y_min = -1;
    let mut ans = 0;
    for (x, y) in schedules.iter() {
        if *x >= y_min {
            ans += 1;
            y_min = *y;
        }
        if *y < y_min {
            y_min = *y;
        }
    }

    println!("{}", ans);
}