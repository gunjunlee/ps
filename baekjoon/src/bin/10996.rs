fn main(){
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    let num: i32 = buffer.trim().parse().unwrap();
    for row in 0..2*num {
        for col in 0..num {
            print!("{}", if (col + row) % 2 == 0 { "*" } else { " " });
        }
        println!();
    }
}
