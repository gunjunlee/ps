macro_rules! parse_line { ($($t: ty),+) => ({
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let mut iter = line.split_whitespace();
    ($(iter.next().unwrap().parse::<$t>().unwrap()),+)
})}

fn main() {
    let (h, w) = parse_line!(i32, i32);
    let mut arr = vec![vec![0i32; w as usize]; h as usize];

    for i in 0..h {
        let mut buf = String::new();
        std::io::stdin().read_line(&mut buf).unwrap();
        let chars: Vec<i32> = buf
            .trim()
            .split("")
            .map(|x| if x == "W" { 0 } else { 1 })
            .collect();
        arr[i as usize] = (&chars[1..(chars.len() - 1)]).to_vec();
    }
    let mut ans = 100;
    for j in 0..(h - 7) {
        for i in 0..(w - 7) {
            let mut conv: i32 = 0;
            for jj in j..(j + 8) {
                for ii in i..(i + 8) {
                    conv += if arr[jj as usize][ii as usize] == 1 {
                        1
                    } else {
                        -1
                    } * if (jj + ii) % 2 == 0 { 1 } else { -1 }
                }
            }
            ans = std::cmp::min(ans, (64 - conv.abs()) / 2);
        }
    }
    println!("{}", ans);
}
