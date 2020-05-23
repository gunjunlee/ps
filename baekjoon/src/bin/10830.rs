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

fn matmul(a: &[[i32; 5]; 5], b: &[[i32; 5]; 5], n: usize) -> [[i32; 5]; 5] {
    let mut res = [[0; 5]; 5];
    for i in 0..n {
        for j in 0..n {
            for k in 0..n {
                res[i][j] = (res[i][j] + a[i][k] * b[k][j]) % 1000;
            }
        }
    }
    return res;
}

fn main() {
    let (n, mut b) = parse_line!(usize, i64);
    let mut a = [[0; 5]; 5];

    for i in 0..n {
        for (idx, &num) in parse_list!(i32).iter().enumerate() {
            a[i][idx] = num % 1000;
        }
    }

    let mut ans = [[0; 5]; 5];
    for i in 0..n {
        ans[i][i] = 1;
    }

    let mut apow = a.clone();

    while b > 0 {
        if b % 2 == 1 {
            ans = matmul(&ans, &apow, n);
        }
        apow = matmul(&apow, &apow, n);
        b = b / 2;
    }

    for i in 0..n {
        for j in 0..n {
            print!("{} ", ans[i][j]);
        }
        println!("");
    }
}
