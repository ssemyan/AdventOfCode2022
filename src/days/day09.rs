use std::collections::HashMap;

use super::Day;

pub fn run_day() {
    let day: Day = Day {
        day_num: String::from("09"),
        part_1_test: String::from("13"),
        part_1: String::from("1798"),
        part_2_test: String::from("8"),
        part_2: String::from("259308"),
    };
    day.run_tests(&run_parts);

    fn run_parts(part_one: bool, lines: &Vec<String>) -> String {
        let mut tail_poslist: HashMap<String,i32> = HashMap::new();

        // follow rope
        let mut h_cur_x = 0;
        let mut h_cur_y = 0;
        let mut t_cur_x = 0;
        let mut t_cur_y = 0;
        
        // Insert the start position
        add_pos(&mut tail_poslist, t_cur_x, t_cur_y);

        for line in lines {

            let dir = &line[0..1];
            let amt: i32 = line[2..].parse::<i32>().unwrap();

            // Move Head horizontally
            let mut hor_mvt = 0;
            let mut vert_mvt = 0;
            match dir {
                "R" => hor_mvt = 1,
                "L" => hor_mvt = -1,
                "U" => vert_mvt = 1,
                "D" => vert_mvt = -1,
                _ => return String::from("Err")
            }
    
            for _ in 0..amt {
                h_cur_x = h_cur_x + hor_mvt;
                h_cur_y = h_cur_y + vert_mvt;
                
                //update tail
                let x_dif = h_cur_x - t_cur_x;
                let y_dif = h_cur_y - t_cur_y;
                if x_dif.abs() > 1 {
                    t_cur_x = t_cur_x + hor_mvt;
                    // may need to move diag
                    t_cur_y = t_cur_y + y_dif;
                } else if y_dif.abs() > 1 {
                    t_cur_y = t_cur_y + vert_mvt;
                    // may need to move diag
                    t_cur_x = t_cur_x + x_dif;
                }
                //println!("H at {}, {}", h_cur_x, h_cur_y);
                //println!("T at {}, {}", t_cur_x, t_cur_y);
                add_pos(&mut tail_poslist, t_cur_x, t_cur_y);
            }
        }

        tail_poslist.len().to_string()
    }
}

fn add_pos(tail_poslist: &mut HashMap<String, i32>, t_cur_x: i32, t_cur_y: i32) {
    let key = format!("{}|{}", t_cur_x, t_cur_y);
    tail_poslist.entry(key).or_insert(1);
}

