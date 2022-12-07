use super::Day;

pub fn run_day() {
    let day: Day = Day {
        day_num: String::from("03"),
        part_1_test: 157,
        part_1: 7763,
        part_2_test: 12,
        part_2: 14979,
    };
    day.run_tests(&run_parts);

    fn run_parts(part_one: bool, lines: &Vec<String>) -> i32 {

        let mut tot_score: i32 = 0;

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
        tot_score
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

fn get_item_val(item: char) -> u32 {
    let mut chr_to_sub = 'a';
    let mut val_to_add = 1;
    if (item as u32) < ('a' as u32) {
        chr_to_sub = 'A';
        val_to_add = 27;
    }
    
    (item as u32) - (chr_to_sub as u32) + val_to_add
}