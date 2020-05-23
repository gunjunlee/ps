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

use std::collections::VecDeque;

fn main() {
    let mut break_directions: Vec<(isize, isize)> = Vec::new();
    let mut nobreak_directions: Vec<(isize, isize)> = Vec::new();
    for x in -2isize..2 {
        for y in -2isize..2 {
            if x.abs() + y.abs() == 2 {
                break_directions.push((x, y));
            }
            if x.abs() + y.abs() == 1 {
                nobreak_directions.push((x, y));
            }
        }
    }
    let (n, m) = parse_line!(isize, isize);
    let mut arr = vec![vec![0i32; m as usize]; n as usize];
    for i in 0..n {
        let mut buffer = String::new();
        std::io::stdin().read_line(&mut buffer).unwrap();
        arr[i as usize] = buffer
            .trim()
            .split("")
            .filter(|x| x != &"")
            .map(|x| x.parse().unwrap())
            .collect();
    }
    let mut cnt = vec![vec![vec![m * n * 2; 2]; m as usize]; n as usize];
    cnt[0][0][0] = 1;
    let mut q: VecDeque<(isize, isize, isize)> = VecDeque::new();
    q.push_back((0, 0, 0));
    while !q.is_empty() {
        let (y, x, is_break) = q.pop_front().unwrap();
        for nobreak_direction in nobreak_directions.iter() {
            let next_y = y - nobreak_direction.0;
            let next_x = x - nobreak_direction.1;
            if next_y >= 0 && next_y < n {
                if next_x >= 0 && next_x < m {
                    if arr[next_y as usize][next_x as usize] == 0 {
                        if cnt[next_y as usize][next_x as usize][is_break as usize]
                            > cnt[y as usize][x as usize][is_break as usize] + 1
                        {
                            cnt[next_y as usize][next_x as usize][is_break as usize] =
                                cnt[y as usize][x as usize][is_break as usize] + 1;
                            q.push_back((next_y, next_x, is_break));
                        }
                    }
                }
            }
        }
        if is_break == 0 {
            for break_direction in break_directions.iter() {
                let next_y = y - break_direction.0;
                let next_x = x - break_direction.1;
                if next_y >= 0 && next_y < n {
                    if next_x >= 0 && next_x < m {
                        if arr[next_y as usize][next_x as usize] == 0 {
                            if cnt[next_y as usize][next_x as usize][1]
                                > cnt[y as usize][x as usize][0] + 2
                            {
                                cnt[next_y as usize][next_x as usize][1] =
                                    cnt[y as usize][x as usize][0] + 2;
                                q.push_back((next_y, next_x, 1));
                            }
                        }
                    }
                }
            }
        }
    }
    let ans = std::cmp::min(
        cnt[n as usize - 1][m as usize - 1][0],
        cnt[n as usize - 1][m as usize - 1][1],
    );
    println!("{}", if ans != m * n * 2 { ans } else { -1 });
}
