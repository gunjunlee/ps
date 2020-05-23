macro_rules! parse_line { ($($t: ty),+) => ({
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let mut iter = line.split_whitespace();
    ($(iter.next().unwrap().parse::<$t>().unwrap()),+)
})}

fn binary_search(lines: &[i64], l: i64, h: i64, target: i64) -> i64 {
    let mut possible = 0;
    if l == h {
        return l;
    }
    let length = (l + h + 1) / 2;
    for line in lines.iter() {
        possible += line / length;
    }
    if possible >= target {
        return binary_search(&lines, length, h, target);
    } else {
        return binary_search(&lines, l, length - 1, target);
    }
}

fn main() {
    let (k, n) = parse_line!(usize, i64);
    let mut lines: Vec<i64> = vec![0; k];
    for i in 0..k {
        lines[i] = parse_line!(i64);
    }
    println!(
        "{}",
        binary_search(&lines, 0, *lines.iter().max().unwrap(), n)
    );
}

// 4 11
// 802
// 743
// 457
// 539
// 200
