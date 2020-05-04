fn main(){
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    let year: u32 = buffer.trim().parse().unwrap();

    if year % 4 == 0 && year % 100 != 0 || year % 400 == 0 {
        println!("1");
    } else {
        println!("0");
    }
}