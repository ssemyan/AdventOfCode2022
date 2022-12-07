use crate::util;
mod day01;
mod day02;
mod day03;

pub fn run_day(day: &str) -> bool {
    
    // Rust does not have reflection so do this nastiness here
    match day {
        "01" => day01::run_day(),
        "02" => day02::run_day(),
        "03" => day03::run_day(),
        
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
    fn run_tests(&self, part_runner: &dyn Fn(bool, &Vec<String>) -> i32) {

        // test file
        let tlines = util::get_file_lines(&self.day_num, true);

        //Part 1
        print_test(true, &self.day_num, true);
        let mut ans = part_runner(true, &tlines);
        println!("Answer: {}", ans);
        assert!(ans == self.part_1_test, "Got wrong answer.");

        // real file
        let rlines = util::get_file_lines(&self.day_num, false);

        print_test(true, &self.day_num, false);
        ans = part_runner(true, &rlines);
        println!("Answer: {}", ans);
        assert!(ans == self.part_1, "Got wrong answer. Expected {}", self.part_1);
    
        // Part 2
        print_test(false, &self.day_num, true);
        ans = part_runner(false, &tlines);
        println!("Answer: {}", ans);
        assert!(ans == self.part_2_test, "Got wrong answer. Expected {}", self.part_2_test);
    
        print_test(false, &self.day_num, false);
        ans = part_runner(false, &rlines);
        println!("Answer: {}", ans);
        assert!(ans == self.part_2, "Got wrong answer. Expected {}", self.part_2);

        println!("Success!!");

        fn print_test(part_one: bool, day: &String, is_test: bool) {
            let mut part = "one";
            if !part_one {
                part = "two";
            }
            println!("Running part {} day {} test {}", part, day, is_test);
        }
    }
}

