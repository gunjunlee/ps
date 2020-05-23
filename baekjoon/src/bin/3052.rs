fn main() {
    let mut cnt = [0; 42];
    for _ in 0..10 {
        let mut buffer = String::new();
        std::io::stdin().read_line(&mut buffer).unwrap();
        cnt[buffer.trim().parse::<usize>().unwrap() % 42] = 1
    }
    println!("{}", cnt.iter().sum::<i32>());
}
