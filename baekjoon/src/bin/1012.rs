macro_rules! parse_line { ($($t: ty),+) => ({
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let mut iter = line.split_whitespace();
    ($(iter.next().unwrap().parse::<$t>().unwrap()),+)
})}

fn main() {
    let t = parse_line!(usize);
    for _ in 0..t {
        let (m, n, k) = parse_line!(usize, usize, usize);
        let mut arr = vec![vec![0; m]; n];

        let mut st: Vec<(usize, usize)> = Vec::new();

        for _ in 0..k {
            let coord = parse_line!(usize, usize);
            arr[coord.1][coord.0] = 1;
        }

        let mut ans = 0;
        for x in 0..m {
            for y in 0..n {
                if arr[y][x] == 1 {
                    ans += 1;
                    let mut st: Vec<(usize, usize)> = Vec::new();
                    st.push((y, x));
                    arr[y][x] = 0;
                    while !st.is_empty() {
                        let (point_y, point_x) = st.pop().unwrap();
                        let directions: Vec<(isize, isize)> =
                            vec![(-1, 0), (1, 0), (0, 1), (0, -1)];
                        for direction in directions.iter() {
                            let next_y = point_y as isize + direction.0;
                            let next_x = point_x as isize + direction.1;
                            if next_y >= 0
                                && next_y < n as isize
                                && next_x >= 0
                                && next_x < m as isize
                            {
                                if arr[next_y as usize][next_x as usize] == 1 {
                                    st.push((next_y as usize, next_x as usize));
                                    arr[next_y as usize][next_x as usize] = 0;
                                }
                            }
                        }
                    }
                }
            }
        }

        println!("{}", ans);
    }
}
