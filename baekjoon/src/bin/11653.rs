macro_rules! parse_line { ($($t: ty),+) => ({
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let mut iter = line.split_whitespace();
    ($(iter.next().unwrap().parse::<$t>().unwrap()),+)
})}

fn main() {
    let mut num = parse_line!(u32);
    let mut div = 2;
    while num > 1 {
        while num % div == 0{
            println!("{}", div);
            num /= div;
        }
        div += 1;
    }
}