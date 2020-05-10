// dirty code

macro_rules! parse_line { ($($t: ty),+) => ({
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let mut iter = line.split_whitespace();
    ($(iter.next().unwrap().parse::<$t>().unwrap()),+)
})}

use std::io::Write;

fn main() {
    let n = parse_line!(i32);
    let mut queue = std::collections::LinkedList::<i32>::new();

    let stdout = std::io::stdout();
    let mut out = std::io::BufWriter::new(stdout.lock());

    for _ in 0..n {
        let mut buf = String::new();
        std::io::stdin().read_line(&mut buf).unwrap();
        let split: Vec<&str> = buf.trim().split(" ").collect();
        match split[0] {
            "push" => {
                queue.push_back(split[1].parse::<i32>().unwrap());
            },
            "pop" => {
                if queue.len() > 0 {
                    writeln!(out, "{}", queue.front().unwrap()).unwrap();
                    queue.pop_front();
                } else {
                    writeln!(out, "{}", -1).unwrap();
                }
            },
            "size" => {
                writeln!(out, "{}", queue.len()).unwrap();
            },
            "empty" => {
                writeln!(out, "{}", if queue.is_empty() {1} else {0}).unwrap();
            },
            "front" => {
                if queue.len() > 0 {
                    writeln!(out, "{}", queue.front().unwrap()).unwrap();
                } else {
                    writeln!(out, "{}", -1).unwrap();
                }

            },
            "back" => {
                if queue.len() > 0 {
                    writeln!(out, "{}", queue.back().unwrap()).unwrap();
                } else {
                    writeln!(out, "{}", -1).unwrap();
                }
            },
            _ => {
                ()
            }
        }
    }
}