use crate::{util, days::print_test};
use super::Day;

pub fn run_day() {
    let day: Day = Day { day_num: String::from("01"), part_1_test: 24000, part_1: 69912, part_2_test: 45000, part_2: 208180};
    day.run_tests(&run_parts);

    fn run_parts(day: &String, part_one: bool, is_test: bool) -> i32 {
        
        print_test(part_one, day, is_test);
        
        // Read file
        let mut lines = util::get_file_lines(day, is_test);

        // Add empty line at the bottom
        lines.push(String::from(""));

        let mut n_max = 0;
        let mut n_cur = 0;
        let mut top_3 = [0,0,0];

        for line in lines {
            // Empty line means next elf
            if line.is_empty() {
                if n_cur > n_max{
                    n_max = n_cur;
                }
                // brute force top 3 check
                if n_cur > top_3[0]{
                    top_3[2] = top_3[1];
                    top_3[1] = top_3[0];
                    top_3[0] = n_cur;
                } else if n_cur > top_3[1]{
                    top_3[2] = top_3[1];
                    top_3[1] = n_cur;
                } else if n_cur > top_3[2]{
                    top_3[2] = n_cur;
                }
                n_cur = 0;
            } else      {
                let x: i32 = line.trim().parse().unwrap();
                n_cur = n_cur + x;
            }
        }
    
        if part_one {
            println!("Max: {}", n_max);
            return n_max;
        } 

        let top_3_sum = top_3[0] + top_3[1] + top_3[2];
        println!("Sum of top 3: {}", top_3_sum);
        top_3_sum

    }
}