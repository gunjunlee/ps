struct Time {
    hour: i32,
    minute: i32,
}

impl Time {
    fn init(hour: i32, minute: i32) -> Time {
        Time {hour: hour, minute: minute}
    }
    fn alarm(&mut self) {
        self.minute -= 45;
        self.cann();
    }
    fn cann(&mut self) {
        self.cann_minute();
        self.cann_hour();
    }
    fn cann_minute(&mut self) {
        if self.minute < 0 {
            self.minute += 60;
            self.hour -= 1;
            self.cann_minute();
        }
    }
    fn cann_hour(&mut self) {
        if self.hour < 0 {
            self.hour += 24;
            self.cann_hour();
        }
    }
}

fn main() {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    let v: Vec<i32> = buffer.trim().split(" ").map(|x| x.trim().parse().unwrap()).collect();
    let mut time: Time = Time::init(v[0], v[1]);
    time.alarm();
    println!("{} {}", time.hour, time.minute);
}