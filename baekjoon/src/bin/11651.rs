macro_rules! parse_line { ($($t: ty),+) => ({
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let mut iter = line.split_whitespace();
    ($(iter.next().unwrap().parse::<$t>().unwrap()),+)
})}

use std::cmp::Ordering::Equal;
use std::io::Write;

fn main(){
    let n = parse_line!(i32);
    let mut coords = Vec::<(i32, i32)>::new();

    let stdout = std::io::stdout();
    let mut out = std::io::BufWriter::new(stdout.lock());

    for _ in 0..n {
        coords.push(parse_line!(i32, i32));
    }
    coords.sort_by(|(x0, y0), (x1, y1)| match y0.cmp(&y1) {
        Equal => x0.cmp(&x1),
        _ => y0.cmp(&y1),
    });
    for i in coords.iter() {
        write!(out, "{} {}\n", i.0, i.1).unwrap();
    }
}