fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let num: i32 = buf.trim().parse().unwrap();
    for i in std::cmp::max(0, num - 100)..num {
        let mut j = i;
        let mut sum = i;
        while j > 0 {
            sum += j % 10;
            j /= 10;
        }
        if sum == num {
            println!("{}", i);
            return;
        }
    }

    println!("{}", 0);
}
