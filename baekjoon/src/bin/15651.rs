macro_rules! parse_line { ($($t: ty),+) => ({
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let mut iter = line.split_whitespace();
    ($(iter.next().unwrap().parse::<$t>().unwrap()),+)
})}

use std::fmt::Write;

fn ordered_combination(
    mut out: &mut String,
    arr: &Vec<i32>,
    mut selected: &mut Vec<i32>,
    mut chars: &mut Vec<usize>,
    num: i32,
) {
    if selected.iter().sum::<i32>() == num {
        for &i in chars.iter() {
            write!(out, "{} ", arr[i]).unwrap();
        }
        writeln!(out, "").unwrap();
        return;
    }
    for i in 0..arr.len() {
        selected[i] += 1;
        chars.push(i);
        ordered_combination(&mut out, &arr, &mut selected, &mut chars, num);
        selected[i] -= 1;
        chars.pop();
    }
}

fn main() {
    let (n, m) = parse_line!(i32, i32);
    let arr = (1..(n + 1)).collect();
    let mut selected = vec![0; n as usize];
    let mut out = String::new();
    let mut chars = Vec::<usize>::new();
    ordered_combination(&mut out, &arr, &mut selected, &mut chars, m);
    println!("{}", out);
}
