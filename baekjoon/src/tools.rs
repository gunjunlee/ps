// read numbers from one line or assert. ex) let (a, b) = parse_line!(i32, i32);
// macro_rules! parse_line { ($($t: ty),+) => ({
//     let mut line = String::new();
//     std::io::stdin().read_line(&mut line).unwrap();
//     let mut iter = line.split_whitespace();
//     ($(iter.next().unwrap().parse::<$t>().unwrap()),+)
// })}

// read numbers from one line or break
// ex) loop {
//     let (a, b) = parse_line!(i32, i32);
// }
// macro_rules! parse_line { ($($t: ty),+) => ({
//     let mut line = String::new();
//     std::io::stdin().read_line(&mut line).unwrap();
//     let mut iter = line.split_whitespace();
//     ($(match iter.next(){
//         Some(num) => num,
//         None => break,
//     }.parse::<$t>().unwrap() ),+)
// })}

// parse list
// macro_rules! parse_list { ($t: ty) => ({
//     let mut line = String::new();
//     std::io::stdin().read_line(&mut line).unwrap();
//     let list: Vec<$t> = line.split_whitespace()
//     .map(|w| w.parse::<$t>().unwrap()).collect(); list
// })}

// #[derive(Debug)]
// struct NumWithInd<T: std::cmp::Ord> {
//     num: T,
//     idx: usize,
// }


// let stdout = std::io::stdout();
// let mut out = std::io::BufWriter::new(stdout.lock());
