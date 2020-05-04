use std::io::Write;

macro_rules! parse_line { ($($t: ty),+) => ({
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let mut iter = line.split_whitespace();
    ($(iter.next().unwrap().parse::<$t>().unwrap()),+)
})}

fn main() {
    let stdout = std::io::stdout();
    let mut out = std::io::BufWriter::new(stdout.lock());

    loop {
        let (a, b) = parse_line!(i32, i32);
        if a == 0 && b == 0 {
            break
        }
        writeln!(out, "{}", a + b).unwrap();
    }
}