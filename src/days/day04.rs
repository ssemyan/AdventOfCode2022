use super::Day;
use regex::Regex;

pub fn run_day() {
    let day: Day = Day {
        day_num: String::from("04"),
        part_1_test: 2,
        part_1: 500,
        part_2_test: 70,
        part_2: 2569
    };
    day.run_tests(&run_parts);

    fn run_parts(part_one: bool, lines: &Vec<String>) -> i32 {
        let mut tot_pairs: i32 = 0;

        if part_one {
            for line in lines {
                // Split into parts
                let re = Regex::new(r"^(\d+)-(\d+),(\d+)-(\d+)$").unwrap();
                let vals = re.captures(line).unwrap();
                let e1_start: i32 = vals.get(1).map_or("", |m| m.as_str()).parse().unwrap(); // OMG is this ugly
                let e1_end: i32 = vals.get(2).map_or("", |m| m.as_str()).parse().unwrap(); 
                let e2_start: i32 = vals.get(3).map_or("", |m| m.as_str()).parse().unwrap(); 
                let e2_end: i32 = vals.get(4).map_or("", |m| m.as_str()).parse().unwrap(); 
                //println!("{} {} {} {}", e1_start, e1_end, e2_start, e2_end);
                
                // Determined if overlapped
                if (e1_start <= e2_start && e1_end >= e2_end) || (e1_start >= e2_start && e1_end <= e2_end) {
                    tot_pairs = tot_pairs + 1;
                }
               
            }
            return tot_pairs;
        }

        // Part two
       

        tot_pairs
    }
}
