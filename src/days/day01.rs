use super::Day;

pub fn run_day() {
    let day: Day = Day {
        day_num: String::from("01"),
        part_1_test: String::from("24000"),
        part_1: String::from("69912"),
        part_2_test: String::from("45000"),
        part_2: String::from("208180")
    };
    day.run_tests(&run_parts);

    fn run_parts(part_one: bool, lines: &Vec<String>) -> String {
        let mut n_max = 0;
        let mut n_cur = 0;
        let mut top_3 = [0, 0, 0];

        let lines_len = lines.len();
        for line_num in 0..(lines_len + 1) {
            // Empty line means next elf
            if line_num == lines_len || lines[line_num].is_empty() {
                if n_cur > n_max {
                    n_max = n_cur;
                }
                // brute force top 3 check
                if n_cur > top_3[0] {
                    top_3[2] = top_3[1];
                    top_3[1] = top_3[0];
                    top_3[0] = n_cur;
                } else if n_cur > top_3[1] {
                    top_3[2] = top_3[1];
                    top_3[1] = n_cur;
                } else if n_cur > top_3[2] {
                    top_3[2] = n_cur;
                }
                n_cur = 0;
            } else if line_num < lines_len {
                let x: i32 = lines[line_num].trim().parse().unwrap();
                n_cur = n_cur + x;
            }
        }

        if part_one {
            return n_max.to_string();
        }

        let top_3_sum = top_3[0] + top_3[1] + top_3[2];
        top_3_sum.to_string()
    }
}
