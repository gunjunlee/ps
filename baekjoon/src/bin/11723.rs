macro_rules! parse_line { ($($t: ty),+) => ({
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let mut iter = line.split_whitespace();
    ($(iter.next().unwrap().parse::<$t>().unwrap()),+)
})}

use std::io::Write;

fn main() {
    let stdout = std::io::stdout();
    let mut out = std::io::BufWriter::new(stdout.lock());

    let n = parse_line!(usize);
    let mut set: u32 = 0;
    for _ in 0..n {
        let mut buffer = String::new();
        std::io::stdin().read_line(&mut buffer).unwrap();
        let buffer = buffer.trim();
        if buffer == "all" {
            set = (2 << 21) - 1;
        } else if buffer == "empty" {
            set = 0;
        } else {
            let buffer: Vec<&str> = buffer.split(" ").collect();
            let command = buffer[0];
            let num = buffer[1];
            let num: u32 = num.trim().parse().unwrap();
            let num: u32 = 2 << (num - 1);
            if command == "add" {
                set = set | num;
            } else if command == "remove" {
                set = if set & num > 0 { set ^ num } else { set };
            } else if command == "check" {
                writeln!(out, "{}", if set & num > 0 { 1 } else { 0 });
            } else if command == "toggle" {
                set = set ^ num;
            }
        }
    }
}
