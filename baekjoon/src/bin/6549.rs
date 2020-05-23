macro_rules! parse_list {
    ($t: ty) => {{
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        let list: Vec<$t> = line
            .split_whitespace()
            .map(|w| w.parse::<$t>().unwrap())
            .collect();
        list
    }};
}

fn main() {
    loop {
        let mut hs = parse_list!(u64);
        if hs.len() == 1 && hs[0] == 0 {
            break;
        }
        hs[0] = 0;
        hs.push(0);

        let mut stack: Vec<(u64, usize)> = Vec::new();
        let mut max: u64 = 0;
        for (idx, &height) in hs.iter().enumerate() {
            while !stack.is_empty() && height < stack[stack.len() - 1].0 {
                let pop = stack.pop().unwrap();
                let i = stack[stack.len() - 1].1;
                let h = pop.0;
                max = std::cmp::max((idx - i - 1) as u64 * h, max);
            }
            stack.push((height, idx));
        }
        println!("{}", max);
    }
}
