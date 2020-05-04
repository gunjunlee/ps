fn main(){
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let num: u32 = buf.trim().parse().unwrap();
    println!("{}", fibonacci(num));

}

fn fibonacci(num: u32) -> u32 {
    if num == 0 {
        0
    } else if num == 1{
        1
    } else {
        fibonacci(num - 1) + fibonacci(num - 2)
    }
}