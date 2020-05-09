macro_rules! parse_line { ($($t: ty),+) => ({
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let mut iter = line.split_whitespace();
    ($(iter.next().unwrap().parse::<$t>().unwrap()),+)
})}

fn main(){
    let num = parse_line!(u32);
    let mut specs = Vec::<(u32, u32)>::new();
    for _ in 0..num {
        let (a, b) = parse_line!(u32, u32);
        specs.push((a, b));
    }
    for i in 0..num {
        let (a, b) = specs[i as usize];
        let mut norder = 0;
        for (pa, pb) in specs.iter() {
            if pa > &a && pb > &b {
                norder += 1;
            }
        }
        print!("{} ", norder+1);
    }
}