use super::Day;
use crate::{days::print_test, util};

pub fn run_day() {
    let day: Day = Day {
        day_num: String::from("02"),
        part_1_test: 15,
        part_1: 12794,
        part_2_test: 12,
        part_2: 14979,
    };
    day.run_tests(&run_parts);

    fn run_parts(day: &String, part_one: bool, is_test: bool) -> i32 {
        print_test(part_one, day, is_test);

        // Read file
        let lines = util::get_file_lines(day, is_test);

        let mut tot_score = 0;

        for line in lines {
            // Split into parts
            let parts: Vec<String> = line.split(" ").map(String::from).collect::<_>();

            let second_part = get_play(part_one, &parts);
            let cur_score = score_for_shape_picked(&second_part) + outcome(&parts[0], &second_part);
            //println!("Score: {}",cur_score);
            tot_score = tot_score + cur_score;
        }

        println!("Total: {}", tot_score);
        tot_score
    }
}

fn get_play(part_one: bool, parts: &Vec<String>) -> String {
    // For part one just play as is
    if part_one {
        return String::from(&parts[1]);
    }

    // For part two look up the play based on the expected outcome
    if parts[1] == "X" {
        // need to lose
        match parts[0].as_str() {
            "A" => String::from("Z"), // rock - scissors
            "B" => String::from("X"), // paper - rock
            "C" => String::from("Y"), // scissors - paper
            _ => panic!("Unknown shape {}", parts[0]),
        }
    } else if parts[1] == "Y" {
        // need to draw
        match parts[0].as_str() {
            "A" => String::from("X"), // rock - rock
            "B" => String::from("Y"), // paper - paper
            "C" => String::from("Z"), // scissors - scissors
            _ => panic!("Unknown shape {}", parts[0]),
        }
    } else {
        // need to win
        match parts[0].as_str() {
            "A" => String::from("Y"), // rock - paper
            "B" => String::from("Z"), // paper - scissors
            "C" => String::from("X"), // scissors - rock
            _ => panic!("Unknown shape {}", parts[0]),
        }
    }
}

fn score_for_shape_picked(shape: &str) -> i32 {
    match shape {
        "X" => 1, // rock
        "Y" => 2, // paper
        "Z" => 3, // scissors
        _ => panic!("Unknown shape {}", shape),
    }
}

fn outcome(shape0: &str, shape1: &str) -> i32 {
    if (shape0 == "A" && shape1 == "X")
        | (shape0 == "B" && shape1 == "Y")
        | (shape0 == "C" && shape1 == "Z")
    {
        return 3; // draw
    }
    if (shape0 == "A" && shape1 == "Z")
        | (shape0 == "B" && shape1 == "X")
        | (shape0 == "C" && shape1 == "Y")
    {
        return 0; // lost
    }
    6 // won
}
