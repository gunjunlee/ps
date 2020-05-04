fn main(){
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let cond: Vec<u32> = buf.trim().split(" ").map(|x| x.trim().parse().unwrap()).collect();
    let num = cond[0] as usize;
    let limit = cond[1];

    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let nums: Vec<u32> = buf.trim().split(" ").map(|x| x.trim().parse().unwrap()).collect();

    let mut ans: u32 = 0;
    for i in 0..num {
        for j in i+1..num {
            for k in j+1..num {
                let sum = nums[i] + nums[j] + nums[k];
                if ans < sum &&  sum <= limit { ans = sum; }
            }
        }
    }
    println!("{}", ans);
}