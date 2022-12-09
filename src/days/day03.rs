use super::Day;

pub fn run_day() {
    let day: Day = Day {
        day_num: String::from("03"),
        part_1_test: String::from("157"),
        part_1: String::from("7763"),
        part_2_test: String::from("70"),
        part_2: String::from("2569")
    };
    day.run_tests(&run_parts);

    fn run_parts(part_one: bool, lines: &Vec<String>) -> String {
        let mut tot_score: i32 = 0;

        if part_one {
            for line in lines {
                // Split into parts
                let poc_len = line.chars().count() / 2;
                let poc0 = &line[..poc_len];
                let poc1 = &line[poc_len..];

                // Determine the part common to both
                let item: char = find_common(poc0, poc1);
                let item_val = get_item_val(item);
                //println!("Found common {} with val {}", item, item_val);
                tot_score = tot_score + (item_val as i32);
            }
            return tot_score.to_string();
        }

        // Part two
        let mut cur_line = 0;
        let line_cnt = lines.len();

        while cur_line < line_cnt {
            
            // Get next 3 lines
            let sack0 = &lines[cur_line];
            let sack1 = &lines[cur_line + 1];
            let sack2 = &lines[cur_line + 2];

            // Determine the part common to both
            let common_1_2 = find_common_all(&sack0, &sack1);
            let item: char = find_common(&common_1_2, &sack2);
            let item_val = get_item_val(item);
            //println!("Found common {} with val {}", item, item_val);
            tot_score = tot_score + (item_val as i32);

            cur_line = cur_line + 3;
        }

        tot_score.to_string()
    }
}

fn find_common(poc0: &str, poc1: &str) -> char {
    // Just going to brute force it here rather than use a lib
    for find_chr in poc0.chars() {
        for cur_chr in poc1.chars() {
            if find_chr == cur_chr {
                return find_chr;
            }
        }
    }
    '\0'
}

fn find_common_all(poc0: &str, poc1: &str) -> String {
    
    let mut char_vec: Vec<char> = Vec::new();
    for find_chr in poc0.chars() {
        for cur_chr in poc1.chars() {
            if find_chr == cur_chr {
                char_vec.push(cur_chr);
                break;
            }
        }
    }
    char_vec.into_iter().collect()
}

fn get_item_val(item: char) -> u32 {
    let mut chr_to_sub = 'a';
    let mut val_to_add = 1;
    if (item as u32) < ('a' as u32) {
        chr_to_sub = 'A';
        val_to_add = 27;
    }

    (item as u32) - (chr_to_sub as u32) + val_to_add
}
