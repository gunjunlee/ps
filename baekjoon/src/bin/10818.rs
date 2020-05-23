fn main() {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    let nums: Vec<i32> = buffer
        .trim()
        .split(" ")
        .map(|x| x.trim().parse().unwrap())
        .collect();
    let min = nums.iter().min().unwrap();
    let max = nums.iter().max().unwrap();
    println!("{} {}", min, max);
}
