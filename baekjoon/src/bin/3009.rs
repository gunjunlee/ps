macro_rules! parse_line { ($($t: ty),+) => ({
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let mut iter = line.split_whitespace();
    ($(iter.next().unwrap().parse::<$t>().unwrap()),+)
})}

fn main(){
    let (mut A, mut B) = (0, 0);
    for _ in 0..3 {
        let (a, b) = parse_line!(i32, i32);
        A ^= a;
        B ^= b;
    }
    println!("{} {}", A, B);
}