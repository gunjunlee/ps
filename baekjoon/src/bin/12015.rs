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

fn main() {
    let _n = parse_line!(usize);
    let nums = parse_list!(i32);
    let mut arr: Vec<i32> = Vec::new();
    arr.push(0);
    for num in nums.iter() {
        match arr.binary_search(&num) {
            Ok(_) => continue,
            Err(i) => {
                if i == arr.len() {
                    arr.push(*num);
                } else {
                    arr[i] = *num;
                }
            }
        }
    }
    println!("{}", arr.len() - 1);
}
