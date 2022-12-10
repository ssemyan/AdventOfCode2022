use super::Day;

pub fn run_day() {
    let day: Day = Day {
        day_num: String::from("06"),
        part_1_test: String::from("11"),
        part_1: String::from("JRVNHHCSJ"),
        part_2_test: String::from("MCD"),
        part_2: String::from("GNFBSBJLH"),
    };
    day.run_tests(&run_parts);

    fn run_parts(part_one: bool, lines: &Vec<String>) -> String {
        
        let mut return_val = 0;

        let line = lines[0].as_bytes();
        let mut cur_char = 3;
        
        loop {
            if cur_char > (line.len() - 1){
                break;
            }

            // Compare prev 3 chars
            if line[cur_char] == line[cur_char - 1] || line[cur_char] == line[cur_char - 2] || line[cur_char] == line[cur_char - 3] 
                || line[cur_char - 1] == line[cur_char - 2] || line[cur_char - 1] == line[cur_char - 3] || line[cur_char - 2] == line[cur_char - 3] {

                    cur_char = cur_char + 1;
                } else {
                    return (cur_char + 1).to_string()
                }            
        }
   
        return_val.to_string()
    }
}
