use super::Day;

pub fn run_day() {
    let day: Day = Day {
        day_num: String::from(""),
        part_1_test: String::from(""),
        part_1: String::from(""),
        part_2_test: String::from(""),
        part_2: String::from(""), 
    };
    day.run_tests(&run_parts);

    fn run_parts(part_one: bool, lines: &Vec<String>) -> String {
        String::from("answer")
    }
}
