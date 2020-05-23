fn main() {
    let (mut max, mut argmax): (i32, i32) = (0, 0);
    for i in 1..10 {
        let mut buf = String::new();
        std::io::stdin().read_line(&mut buf).unwrap();
        let num: i32 = buf.trim().parse().unwrap();
        if num > max {
            max = num;
            argmax = i;
        }
    }
    println!("{}\n{}", max, argmax);
}
