macro_rules! parse_line { ($($t: ty),+) => ({
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let mut iter = line.split_whitespace();
    ($(iter.next().unwrap().parse::<$t>().unwrap()),+)
})}

use std::collections::LinkedList;

fn main() {
    let n = parse_line!(i32);
    let mut queue: LinkedList<i32> = LinkedList::new();
    for i in 1..n + 1 {
        queue.push_back(i);
    }
    while queue.len() > 1 {
        queue.pop_front();
        let front = queue.pop_front().unwrap();
        queue.push_back(front);
    }
    println!("{}", queue.front().unwrap());
}
