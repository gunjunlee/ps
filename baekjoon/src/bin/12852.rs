macro_rules! parse_line { ($($t: ty),+) => ({
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let mut iter = line.split_whitespace();
    ($(iter.next().unwrap().parse::<$t>().unwrap()),+)
})}

use std::collections::LinkedList;

fn main() {
    let n = parse_line!(usize);
    let mut dp: Vec<usize> = vec![0; n + 1];
    let mut queue: LinkedList<usize> = LinkedList::new();
    queue.push_back(n);
    loop {
        let m = queue.pop_front().unwrap();
        if m == 1 {
            break;
        }
        if m % 2 == 0 {
            if dp[m / 2] == 0 && m / 2 >= 1 {
                dp[m / 2] = m;
                queue.push_back(m / 2);
            }
        }
        if m % 3 == 0 {
            if dp[m / 3] == 0 && m / 3 >= 1 {
                dp[m / 3] = m;
                queue.push_back(m / 3);
            }
        }
        if dp[m - 1] == 0 && m - 1 >= 1 {
            dp[m - 1] = m;
            queue.push_back(m - 1);
        }
    }
    let mut list: Vec<usize> = Vec::new();
    let mut m = 1;
    loop {
        list.push(m);
        if m == n {
            break;
        }
        m = dp[m];
    }
    println!("{}", list.len() - 1);
    for iter in list.iter().rev() {
        print!("{} ", iter);
    }
}
