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

macro_rules! parse_line { ($($t: ty),+) => ({
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let mut iter = line.split_whitespace();
    ($(iter.next().unwrap().parse::<$t>().unwrap()),+)
})}

fn get_min_max(
    nums: &Vec<i32>,
    adds: i32,
    minuses: i32,
    muls: i32,
    divs: i32,
    acc: i32,
    loc: usize,
) -> (i32, i32) {
    if loc == nums.len() {
        return (acc, acc);
    }
    let mut minmax = (i32::max_value(), i32::min_value());
    let next = nums[loc];
    if adds > 0 {
        let output = get_min_max(&nums, adds - 1, minuses, muls, divs, acc + next, loc + 1);
        minmax.0 = std::cmp::min(minmax.0, output.0);
        minmax.1 = std::cmp::max(minmax.1, output.1);
    }
    if minuses > 0 {
        let output = get_min_max(&nums, adds, minuses - 1, muls, divs, acc - next, loc + 1);
        minmax.0 = std::cmp::min(minmax.0, output.0);
        minmax.1 = std::cmp::max(minmax.1, output.1);
    }
    if muls > 0 {
        let output = get_min_max(&nums, adds, minuses, muls - 1, divs, acc * next, loc + 1);
        minmax.0 = std::cmp::min(minmax.0, output.0);
        minmax.1 = std::cmp::max(minmax.1, output.1);
    }
    if divs > 0 {
        let output = get_min_max(&nums, adds, minuses, muls, divs - 1, acc / next, loc + 1);
        minmax.0 = std::cmp::min(minmax.0, output.0);
        minmax.1 = std::cmp::max(minmax.1, output.1);
    }
    return minmax;
}

fn main() {
    parse_line!(i32);
    let nums = parse_list!(i32);
    let (adds, minuses, muls, divs) = parse_line!(i32, i32, i32, i32);

    let acc = nums[0];
    let minmax = get_min_max(&nums, adds, minuses, muls, divs, acc, 1);
    println!("{}\n{}", minmax.1, minmax.0);
}
