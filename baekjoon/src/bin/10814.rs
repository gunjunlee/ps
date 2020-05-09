macro_rules! parse_line { ($($t: ty),+) => ({
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let mut iter = line.split_whitespace();
    ($(iter.next().unwrap().parse::<$t>().unwrap()),+)
})}

use std::io::Write;

fn main(){
    let n = parse_line!(i32);
    let mut infos = Vec::<(i32, String)>::new();

    let stdout = std::io::stdout();
    let mut out = std::io::BufWriter::new(stdout.lock());

    for _ in 0..n {
        infos.push(parse_line!(i32, String));
    }
    infos.sort_by(|(y0, _n0), (y1, _n1)| y0.cmp(&y1));
    for i in infos.iter() {
        write!(out, "{} {}\n", i.0, i.1).unwrap();
    }
}