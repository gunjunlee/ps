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

fn count_color(arr: &Vec<Vec<i32>>, x: usize, y: usize, n: usize) -> (i32, i32) {
    let color = arr[x][y];
    let mut good = true;
    'Check: for i in x..x + n {
        for j in y..y + n {
            if arr[i][j] != color {
                good = false;
                break 'Check;
            }
        }
    }
    if good {
        if color == 0 {
            return (1, 0);
        } else {
            return (0, 1);
        }
    }

    let (mut white, mut blue): (i32, i32) = (0, 0);

    for &i in [x, x + n / 2].iter() {
        for &j in [y, y + n / 2].iter() {
            let white_blue = count_color(&arr, i, j, n / 2);
            white += white_blue.0;
            blue += white_blue.1;
        }
    }
    (white, blue)
}

fn main() {
    let n = parse_line!(usize);
    let mut arr: Vec<Vec<i32>> = vec![vec![0; n]; n];

    for i in 0..n {
        arr[i] = parse_list!(i32);
    }

    let (white, blue) = count_color(&arr, 0, 0, n);

    println!("{}\n{}", white, blue);
}
