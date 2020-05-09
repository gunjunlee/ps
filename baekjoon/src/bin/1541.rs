fn parse(exp: &str) -> (Vec::<i32>, Vec::<char>) {
    let mut start = 0;
    let mut nums = Vec::<&str>::new();
    let mut terms = Vec::<char>::new();
    for (i, ch) in exp.chars().enumerate() {
        if ch == '+' || ch == '-' {
            nums.push(&exp[start..i]);
            start = i + 1;
            terms.push(ch);
        }
    }
    nums.push(&exp[start..exp.len()]);
    let nums = nums.iter().map(|x| x.trim().parse::<i32>().unwrap()).collect();
    return (nums, terms);
}

fn main() {
    let mut exp = String::new();
    std::io::stdin().read_line(&mut exp).unwrap();
    let (nums, terms) = parse(&exp);
    let mut ans = nums[0];
    let mut is_minus = false;
    for i in 0..terms.len() {
        if terms[i] == '-' {
            is_minus = true;
        }
        if is_minus {
            ans -= nums[i + 1];
        } else {
            ans += nums[i + 1];
        }
    }
    println!("{}", ans);
}