macro_rules! parse_line { ($($t: ty),+) => ({
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let mut iter = line.split_whitespace();
    ($(iter.next().unwrap().parse::<$t>().unwrap()),+)
})}

fn main() {
    loop {
        let (a, b) = parse_line!(i32, i32);
        if a ^ b == 0 {
            break;
        }
        if a % b == 0 {
            println!("multiple");
        } else if b % a == 0 {
            println!("factor");
        } else {
            println!("neither");
        }
    }
}