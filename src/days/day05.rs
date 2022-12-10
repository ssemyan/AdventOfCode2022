use super::Day;
use regex::Regex;

pub fn run_day() {
    let day: Day = Day {
        day_num: String::from("05"),
        part_1_test: String::from("CMZ"),
        part_1: String::from("JRVNHHCSJ"),
        part_2_test: String::from("MCD"),
        part_2: String::from("GNFBSBJLH"),
    };
    day.run_tests(&run_parts);

    fn run_parts(part_one: bool, lines: &Vec<String>) -> String {
        
        let mut return_val = String::new();

        // Build stacks
        // [D]    
        // [N] [C]    
        // [Z] [M] [P]
        //  1   2   3 

        let mut stacks: Vec<Vec<char>> = Vec::new();

        let mut line_num = 0;
        let mut first_row: bool = true;

        // First create the stacks
        loop {
            let line = lines[line_num].as_bytes();
            if line[1] as char == '1' {
                // Done with stacks
                break;
            }

            // Walk through the line, every 4 characters
            let mut char_num = 1;
            let mut stack_num = 0;
            loop {
                if char_num > line.len() - 1 {
                    // At end of line
                    break;
                }

                // Create stack if first row
                if first_row {
                    stacks.push(Vec::new());
                }
                
                if line[char_num] as char != ' ' {
                    stacks[stack_num].push(line[char_num] as char);
                } 
                stack_num = stack_num + 1;
                char_num = char_num + 4;
            }
            line_num = line_num + 1;
            first_row = false;
        }

        // Print top of each stack
        // for stack in &stacks {
        //     println!("{} at top of {} items", stack[0], stack.len());
        // }
        
        // Now do the moves
        line_num = line_num + 2;
        let re = Regex::new(r"^move (\d+) from (\d+) to (\d+)$").unwrap();
        loop {
            if line_num >= lines.len() {
                break;
            }
            let vals = re.captures(&lines[line_num]).unwrap();
            let num_to_move: i32 = vals.get(1).map_or("", |m| m.as_str()).parse().unwrap(); // OMG is this ugly
            let move_from: usize = vals.get(2).map_or("", |m| m.as_str()).parse::<usize>().unwrap() - 1; // OMG is this ugly
            let move_to: usize = vals.get(3).map_or("", |m| m.as_str()).parse::<usize>().unwrap() - 1; // OMG is this ugly
            //println!("Moving {} from {} to {}", num_to_move, move_from, move_to);

            for idx in 0..num_to_move {
                let element: char = stacks[move_from][0].clone();
                if part_one {
                    stacks[move_to].insert(0, element);
                } else {
                    stacks[move_to].insert(idx as usize, element);
                }
            
                stacks[move_from].remove(0);
            }

            line_num = line_num + 1;
        }

        // Print top of each stack
        for stack in &stacks {
            //print!("{}", stack[0]);
            return_val.push(stack[0]);
        }
   
        return_val
    }
}
