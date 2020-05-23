macro_rules! parse_line { ($($t: ty),+) => ({
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let mut iter = line.split_whitespace();
    ($(iter.next().unwrap().parse::<$t>().unwrap()),+)
})}

macro_rules! parse_list {
    ($t: ty) => {{
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        let list: Vec<$t> = line
            .split_whitespace()
            .map(|w| w.parse::<$t>().unwrap())
            .collect();
        list
    }};
}

use std::io::Write;

fn lotto<T: std::io::Write>(mut out: &mut std::io::BufWriter<T>, arr: &[i32]) {
    fn combinations<T: std::io::Write>(
        mut out: &mut std::io::BufWriter<T>,
        arr: &[i32],
        mut checked: &mut Vec<bool>,
        index: usize,
    ) {
        if checked.iter().map(|x| *x as usize).sum::<usize>() == 6 {
            for (idx, check) in checked.iter().enumerate() {
                if *check {
                    write!(out, "{} ", arr[idx]);
                }
            }
            writeln!(out);
            return;
        } else if index == arr.len() {
            return;
        }
        checked[index] = true;
        combinations(&mut out, &arr, &mut checked, index + 1);
        checked[index] = false;
        combinations(&mut out, &arr, &mut checked, index + 1);
    }
    let mut checked = vec![false; arr.len()];
    combinations(&mut out, &arr, &mut checked, 0);
}

fn main() {
    let stdout = std::io::stdout();
    let mut out = std::io::BufWriter::new(stdout.lock());

    loop {
        let arr = parse_list!(i32);
        if arr.len() == 1 {
            break;
        }
        lotto(&mut out, &arr[1..]);
        writeln!(out);
    }
}
