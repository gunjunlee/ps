fn main() {
    let mut inp = String::new();
    std::io::stdin().read_line(&mut inp).unwrap();
    let parts: Vec<i32> = inp.split(' ').map(|x| x.trim().parse().unwrap()).collect();
    if parts[0] == parts[1] {
        println!("==")
    }
    else if parts[0] < parts[1]{
        println!("<")
    }
    else {
        println!(">")
    }
}