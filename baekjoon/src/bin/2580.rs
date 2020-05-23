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

fn solve_sudoku(mut arr: &mut Vec<Vec<i32>>, x: usize, y: usize) -> bool {
    let (mut xx, mut yy) = (x, y);
    loop {
        if arr[xx][yy] != 0 {
            yy += 1;
            if yy == 9 {
                yy = 0;
                xx += 1;
                if xx == 9 {
                    return true;
                }
            }
        } else {
            break;
        }
    }
    let mut exists = [0; 10];
    for i in 0..9 {
        exists[arr[xx][i] as usize] = 1;
        exists[arr[i][yy] as usize] = 1;
    }
    let batch_x = (xx / 3) * 3;
    let batch_y = (yy / 3) * 3;
    for i in batch_x..batch_x + 3 {
        for j in batch_y..batch_y + 3 {
            exists[arr[i][j] as usize] = 1;
        }
    }
    for (num, &exist) in exists.iter().enumerate() {
        if exist == 0 {
            arr[xx][yy] = num as i32;
            if solve_sudoku(&mut arr, xx, yy) {
                return true;
            }
        }
    }
    arr[xx][yy] = 0;
    return false;
}

fn main() {
    let mut arr = vec![vec![0; 9]; 9];
    for i in 0..9 {
        arr[i] = parse_list!(i32);
    }
    solve_sudoku(&mut arr, 0, 0);
    for i in arr.iter() {
        for j in i.iter() {
            print!("{} ", j);
        }
        println!("");
    }
}

// 0 0 4 6 0 0 2 8 0
// 0 1 0 0 8 2 0 0 7
// 0 5 0 0 3 9 0 0 6
// 0 0 8 3 0 0 1 6 0
// 0 9 7 0 0 0 0 0 0
// 4 0 0 2 0 8 9 0 0
// 3 0 0 1 4 0 0 9 0
// 0 7 9 0 2 0 0 4 0
// 0 0 0 0 0 5 6 0 0

// 0 0 5 3 0 0 0 0 0
// 8 0 0 0 0 0 0 2 0
// 0 7 0 0 1 0 5 0 0
// 4 0 0 0 0 5 3 0 0
// 0 1 0 0 7 0 0 0 6
// 0 0 3 2 0 0 0 8 0
// 0 6 0 5 0 0 0 0 9
// 0 0 4 0 0 0 0 3 0
// 0 0 0 0 0 9 7 0 0
