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

fn binary_left(nums: &[i32], q: i32) -> Result<usize, ()> {
    fn _binary_left(nums: &[i32], q: i32, n: usize) -> Result<usize, ()> {
        if n == 0 {
            return Err(());
        } else if n == 1 {
            if nums[0] == q {
                return Ok(0);
            } else {
                return Err(());
            }
        }
        let line = (n - 1) / 2;
        if q <= nums[line] {
            return _binary_left(&nums[0..line + 1], q, line + 1);
        } else {
            return match _binary_left(&nums[line + 1..n], q, n - line - 1) {
                Ok(i) => Ok(i + line + 1),
                Err(_) => Err(()),
            };
        }
    }
    let n = nums.len();
    return _binary_left(&nums, q, n);
}

fn binary_right(nums: &[i32], q: i32) -> Result<usize, ()> {
    fn _binary_right(nums: &[i32], q: i32, n: usize) -> Result<usize, ()> {
        if n == 0 {
            return Err(());
        } else if n == 1 {
            if nums[0] == q {
                return Ok(0);
            } else {
                return Err(());
            }
        }
        let line = n / 2;
        if q < nums[line] {
            return _binary_right(&nums[0..line], q, line);
        } else {
            return match _binary_right(&nums[line..n], q, n - line) {
                Ok(i) => Ok(i + line),
                Err(_) => Err(()),
            };
        }
    }
    let n = nums.len();
    return _binary_right(&nums, q, n);
}

use std::io::Write;

fn main() {
    let stdout = std::io::stdout();
    let mut out = std::io::BufWriter::new(stdout.lock());

    let _n = parse_line!(usize);
    let mut nums = parse_list!(i32);
    let _m = parse_line!(usize);
    let queries = parse_list!(i32);
    nums.sort();
    for &query in queries.iter() {
        let left = binary_left(&nums, query);
        let right = binary_right(&nums, query);
        match left {
            Ok(_) => write!(out, "{} ", right.unwrap() - left.unwrap() + 1).unwrap(),
            Err(_) => write!(out, "0 ").unwrap(),
        }
    }
}
