macro_rules! parse_list { ($t: ty) => ({
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let list: Vec<$t> = line.split_whitespace()
    .map(|w| w.parse::<$t>().unwrap()).collect(); list
})}

macro_rules! parse_line { ($($t: ty),+) => ({
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let mut iter = line.split_whitespace();
    ($(iter.next().unwrap().parse::<$t>().unwrap()),+)
})}

fn dfs(n: i32, rel_sums: &Vec<Vec<i32>>, loc: usize, mut selected: &mut Vec<usize>) -> i32 {
    if n == 0 {
        let mut selected_sums = 0;
        for i in 0..selected.len() {
            for j in (i+1)..selected.len() {
                selected_sums += rel_sums[selected[i] as usize][selected[j] as usize];
            }
        }
        let mut unselected = Vec::<usize>::new();
        for i in 0..rel_sums.len() {
            if !selected.contains(&i) {
                unselected.push(i);
            }
        }
        let mut unselected_sums = 0;
        for i in 0..unselected.len() {
            for j in (i+1)..unselected.len() {
                unselected_sums += rel_sums[unselected[i] as usize][unselected[j] as usize];
            }
        }
        return (unselected_sums - selected_sums).abs();
    }
    let mut min = i32::max_value();
    for l in loc..rel_sums.len() {
        if rel_sums.len() >= l + n as usize - 1 {
            selected.push(l);
            let diff = dfs(n-1, &rel_sums, l + 1, &mut selected);
            selected.pop();
            min = std::cmp::min(diff, min);
        }
    }
    return min;
}

fn main() {
    let n  = parse_line!(i32);
    let mut rels = Vec::<Vec<i32>>::new();
    for _ in 0..n {
        rels.push(parse_list!(i32));
    }
    let mut rel_sums = vec![vec![0; n as usize]; n as usize];
    for i in 0..rel_sums.len() {
        for j in i+1..rel_sums.len() {
            rel_sums[i][j] = rels[i][j] + rels[j][i];
            rel_sums[j][i] = rels[i][j] + rels[j][i];
        }
    }
    let mut selected = Vec::<usize>::new();
    println!("{}", dfs(n / 2, &rel_sums, 0, &mut selected));
}