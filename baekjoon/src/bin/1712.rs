macro_rules! parse_line { ($($t: ty),+) => ({
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let mut iter = line.split_whitespace();
    ($(iter.next().unwrap().parse::<$t>().unwrap()),+)
})}

fn main(){
    let (a, b, c) = parse_line!(i32, i32, i32);
    if b >= c {
        println!("-1");
    } else {
        println!("{}", a / (c-b) + 1);
    }
}