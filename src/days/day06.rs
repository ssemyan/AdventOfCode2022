use super::Day;

pub fn run_day() {
    let day: Day = Day {
        day_num: String::from("06"),
        part_1_test: String::from("11"),
        part_1: String::from("1760"),
        part_2_test: String::from("26"),
        part_2: String::from("2974"),
    };
    day.run_tests(&run_parts);

    fn run_parts(part_one: bool, lines: &Vec<String>) -> String {

        let line = lines[0].as_bytes();
        let mut num_unique: usize = 4;
        if !part_one {
            num_unique = 14;
        }
        let mut cur_char = num_unique - 1;

        loop {
            if cur_char > (line.len() - 1) {
                break;
            }

            // Compare prev N chars
            let is_unique = is_unique(num_unique, &line[cur_char - (num_unique - 1)..cur_char + 1]);
            if is_unique {
                return (cur_char + 1).to_string();
            } else {
                cur_char = cur_char + 1;
            }
        }

        String::from("Failed to find header")
    }

    fn is_unique(num_chars: usize, line: &[u8]) -> bool {
        let l_size = line.len();
        if l_size == 1 {
            return true;
        }

        // Verify curr st
        for id in 1..l_size {
            if line[0] == line[id] {
                return false;
            }
        }

        // Recurse for the rest of the line
        is_unique(num_chars, &line[1..])
    }
}
