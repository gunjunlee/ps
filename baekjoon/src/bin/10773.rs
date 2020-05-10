macro_rules! parse_line { ($($t: ty),+) => ({
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let mut iter = line.split_whitespace();
    ($(iter.next().unwrap().parse::<$t>().unwrap()),+)
})}

fn main() {
    let mut stack = Vec::<i32>::new();
    let n = parse_line!(i32);
    for _ in 0..n {
        let inp = parse_line!(i32);
        if inp != 0 {
            stack.push(inp);
        } else {
            stack.pop();
        }
    }
    println!("{}", stack.iter().sum::<i32>());
}