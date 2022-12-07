mod day01;
mod day02;

pub fn run_day(day: &str) -> bool {
    
    // Rust does not have reflection so do this nastiness here
    match day {
        "01" => day01::run_day(),
        "02" => day02::run_day(),
        
        _ => return false
    }
    true
}

struct Day {
    day_num: String,
    part_1: i32,
    part_1_test: i32,
    part_2: i32,
    part_2_test: i32
}

impl Day {
    fn run_tests(&self, part_runner: &dyn Fn(&String, bool, bool) -> i32) {
        //Part 1
        let mut ans = part_runner(&self.day_num, true, true);
        assert!(ans == self.part_1_test, "Got wrong answer.");
    
        ans = part_runner(&self.day_num, true, false);
        assert!(ans == self.part_1, "Got wrong answer.");
    
        // Part 2
        ans = part_runner(&self.day_num, false, true);
        assert!(ans == self.part_2_test, "Got wrong answer.");
    
        ans = part_runner(&self.day_num, false, false);
        assert!(ans == self.part_2, "Got wrong answer.");

        println!("Success!!");
    }
}

fn print_test(part_one: bool, day: &String, is_test: bool) {
    let mut part = "one";
    if !part_one {
        part = "two";
    }
    println!("Running part {} day {} test {}", part, day, is_test);
}