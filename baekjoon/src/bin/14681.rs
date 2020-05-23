fn read_number() -> i32 {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    buffer.trim().parse::<i32>().unwrap()
}

fn main() {
    let a = read_number();
    let b = read_number();
    if a > 0 {
        if b > 0 {
            println!("1")
        } else {
            println!("4")
        }
    } else {
        if b > 0 {
            println!("2")
        } else {
            println!("3")
        }
    }
}
