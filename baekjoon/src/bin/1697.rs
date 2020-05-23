macro_rules! parse_line { ($($t: ty),+) => ({
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let mut iter = line.split_whitespace();
    ($(iter.next().unwrap().parse::<$t>().unwrap()),+)
})}

use std::collections::VecDeque;

fn main() {
    let (n, k) = parse_line!(i32, i32);
    let mut cnt = vec![-1; 200001];
    let mut q: VecDeque<i32> = VecDeque::new();
    q.push_back(n);
    cnt[n as usize] = 0;
    loop {
        let num = q.pop_front().unwrap();
        if num == k {
            println!("{}", cnt[num as usize]);
            break;
        }
        if num - 1 >= 0 && cnt[num as usize - 1] == -1 {
            cnt[num as usize - 1] = cnt[num as usize] + 1;
            q.push_back(num - 1);
        }
        if num + 1 <= 200000 && cnt[num as usize + 1] == -1 {
            cnt[num as usize + 1] = cnt[num as usize] + 1;
            q.push_back(num + 1);
        }
        if num * 2 <= 200000 && cnt[num as usize * 2] == -1 {
            cnt[num as usize * 2] = cnt[num as usize] + 1;
            q.push_back(num * 2);
        }
    }
}
