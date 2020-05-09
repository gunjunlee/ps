macro_rules! parse_line { ($($t: ty),+) => ({
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let mut iter = line.split_whitespace();
    ($(iter.next().unwrap().parse::<$t>().unwrap()),+)
})}

use std::io::Write;

fn comb<T: Write>(
    mut out: &mut std::io::BufWriter<T>,
    mut selected: &mut Vec<u32>,
    mut chars: &mut Vec<i32>,
    arr: &Vec<u32>,
    len: u32,
) {
    if selected.iter().sum::<u32>() == len {
        for i in chars.iter() {
            write!(out, "{} ", arr[*i as usize]).unwrap();
        }
        writeln!(out, "").unwrap();
    }
    for i in 0..selected.len() {
        if selected[i] == 1 {
            continue;
        }
        selected[i] = 1;
        chars.push(i as i32);
        comb(&mut out, &mut selected, &mut chars, &arr, len);
        selected[i] = 0;
        chars.pop();
    }
}

fn main() {
    let stdout = std::io::stdout();
    let mut out = std::io::BufWriter::new(stdout.lock());

    let (n, m) = parse_line!(u32, u32);
    let vec = (1..(n + 1)).collect();
    let mut _selected = vec![0; n as usize];
    let mut _chars = Vec::<i32>::new();
    comb(&mut out, &mut _selected, &mut _chars, &vec, m);
}
