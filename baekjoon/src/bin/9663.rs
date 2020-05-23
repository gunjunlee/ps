fn poo(mut pos: &mut [[i32; 17]; 17], x: usize, size: usize) -> i32 {
    if x == size {
        return 1;
    }
    let mut ans: i32 = 0;
    for i in 0..size {
        if pos[x][i] == 0 {
            for j in 1..size {
                if x + j >= size {
                    break;
                }
                if i >= j {
                    pos[x + j][i - j] += 1;
                }
                if i + j < size {
                    pos[x + j][i + j] += 1;
                }
                pos[x + j][i] += 1;
            }
            ans += poo(&mut pos, x + 1, size);
            for j in 1..size {
                if x + j >= size {
                    break;
                }
                if i >= j {
                    pos[x + j][i - j] -= 1;
                }
                if i + j < size {
                    pos[x + j][i + j] -= 1;
                }
                pos[x + j][i] -= 1;
            }
        }
    }
    ans
}

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let size: i32 = buf.trim().parse().unwrap();

    let mut pos = [[0; 17]; 17];

    let ans = poo(&mut pos, 0, size as usize);
    println!("{}", ans);
}
