macro_rules! parse_line { ($($t: ty),+) => ({
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let mut iter = line.split_whitespace();
    ($(iter.next().unwrap().parse::<$t>().unwrap()),+)
})}

fn search(xs: &[i64], l: i64, h: i64, c: usize) -> i64 {
    if l == h {
        return l;
    }

    let mut last = xs[0];
    let mut cnt = 1;
    let line = (l + h + 1) / 2;
    for idx in 0..xs.len() {
        if line <= xs[idx] - last {
            last = xs[idx];
            cnt += 1;
        }
    }
    if cnt < c {
        return search(&xs, l, line - 1, c);
    } else {
        return search(&xs, line, h, c);
    }
}

fn main() {
    let (n, c) = parse_line!(usize, usize);
    let mut xs: Vec<i64> = vec![0; n];
    for i in 0..n {
        xs[i] = parse_line!(i64);
    }
    xs.sort();
    println!("{}", search(&xs, 0, i32::max_value() as i64, c));
}

// 5 3
// 1
// 2
// 8
// 4
// 9
