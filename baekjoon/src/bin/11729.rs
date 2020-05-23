use std::io::Write;

fn hanoi<T: std::io::Write>(
    mut out: std::io::BufWriter<T>,
    level: u32,
    from: u32,
    to: u32,
) -> std::io::BufWriter<T> {
    if level > 1 {
        out = hanoi(out, level - 1, from, 3 - from - to)
    };
    write!(out, "{} {}\n", from + 1, to + 1).unwrap();
    if level > 1 {
        out = hanoi(out, level - 1, 3 - from - to, to)
    };
    return out;
}

fn main() {
    let stdout = std::io::stdout();
    let out = std::io::BufWriter::new(stdout.lock());

    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let num: u32 = buf.trim().parse().unwrap();
    println!("{}", (2 << (num - 1)) - 1);
    hanoi(out, num, 0, 2);
}
