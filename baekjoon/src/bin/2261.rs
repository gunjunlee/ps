macro_rules! parse_line { ($($t: ty),+) => ({
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let mut iter = line.split_whitespace();
    ($(iter.next().unwrap().parse::<$t>().unwrap()),+)
})}

struct Point(i32, i32);

impl Point {
    fn distance(&self, p: &Point) -> i32 {
        let x = self.0 - p.0;
        let y = self.1 - p.1;
        return x * x + y * y;
    }
}

fn get_min(mut points: &mut [Point], n: usize) -> i32 {
    let n = points.len();
    if n == 2 {
        return points[0].distance(&points[1]);
    } else if n == 3 {
        return std::cmp::min(
            std::cmp::min(
                points[0].distance(&points[1]),
                points[0].distance(&points[2]),
            ),
            points[1].distance(&points[2]),
        );
    }

    let line = (points[n / 2].0 + points[n / 2 - 1].0) / 2;
    let mut d = std::cmp::min(
        get_min(&mut points[0..n / 2], n / 2),
        get_min(&mut points[n / 2..n], n - n / 2),
    );
    let mut mid: Vec<&Point> = Vec::new();
    for point in points.iter() {
        let t = point.0 - line;
        if t * t < d {
            mid.push(point);
        }
    }
    mid.sort_by(|p1, p2| p1.1.cmp(&p2.1));
    for i in 0..mid.len() {
        for j in i + 1..mid.len() {
            let distance = (mid[i].1 - mid[j].1) * (mid[i].1 - mid[j].1);
            if distance >= d {
                break;
            }
            d = std::cmp::min(mid[i].distance(&mid[j]), d);
        }
    }
    return d;
}

fn main() {
    let n = parse_line!(usize);
    let mut points: Vec<Point> = Vec::new();
    for _ in 0..n {
        let (x, y) = parse_line!(i32, i32);
        points.push(Point(x, y));
    }
    points.sort_by(|p1, p2| p1.0.cmp(&p2.0));
    let n = points.len();
    let d = get_min(&mut points, n);
    println!("{}", d);
}
