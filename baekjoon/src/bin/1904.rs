fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let n: usize = buf.trim().parse().unwrap();
    let mut dp = vec![1, 1];
    for i in 2..(n + 1) {
        dp.push((dp[i - 2] + dp[i - 1]) % 15746);
    }
    println!("{}", dp[n]);
}
