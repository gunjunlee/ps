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

fn main() {
    let _ = parse_list!(u32);
    let mut arr = parse_list!(u32);
    arr.sort();
    println!("{}", arr[0] * arr[arr.len() - 1]);
}
