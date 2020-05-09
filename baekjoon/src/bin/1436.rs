fn main(){
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let mut num: i32 = buf.trim().parse().unwrap();
    let mut title: i32 = 0;
    while num > 0 {
        title += 1;
        if title.to_string().contains("666") {
            num -= 1;
        }
    }
    println!("{}", title);
}