fn main() {
    loop {
        let mut buf = String::new();
        std::io::stdin().read_line(&mut buf).unwrap();
        let mut stack = Vec::<char>::new();
        let mut breaked = false;
        for (idx, ch) in buf.chars().enumerate() {
            if idx == 0 && ch == '.' {
                return ();
            }
            if ['(', '{', '['].contains(&ch) {
                stack.push(ch);
            } else if ch == ')' {
                match stack.pop() {
                    Some('(') => (),
                    _ => {
                        println!("no");
                        breaked = true;
                        break;
                    }
                }
            } else if ch == '}' {
                match stack.pop() {
                    Some('{') => (),
                    _ => {
                        println!("no");
                        breaked = true;
                        break;
                    }
                }
            } else if ch == ']' {
                match stack.pop() {
                    Some('[') => (),
                    _ => {
                        println!("no");
                        breaked = true;
                        break;
                    }
                }
            }
        }
        if !breaked {
            if stack.len() > 0 {
                println!("no");
            } else {
                println!("yes");
            }
        }
    }
}
