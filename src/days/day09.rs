use std::collections::HashMap;

use super::Day;

pub fn run_day() {
    let day: Day = Day {
        day_num: String::from("09"),
        part_1_test: String::from("88"),
        part_1: String::from("6367"),
        part_2_test: String::from("36"),
        part_2: String::from("2536"), 
    };
    day.run_tests(&run_parts);

    fn run_parts(part_one: bool, lines: &Vec<String>) -> String {
        let mut tail_poslist: HashMap<String,i32> = HashMap::new();

        // will have many knots in part two
        let mut ends: Vec<Point> = Vec::new();

        // Add head and tail(s)
        let mut end_tail = 2;
        if !part_one {
            end_tail = 10; // 9 tails plus the head
        }
        for _ in 0..end_tail {
            ends.push(Point{ x: 0, y: 0});
        }

        // Process instructions
        for line in lines {

            let dir = &line[0..1];
            let amt: i32 = line[2..].parse::<i32>().unwrap();

            // Move Head
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

                // Move head
                ends[0].x = ends[0].x + hor_mvt;
                ends[0].y = ends[0].y + vert_mvt;
                
                //update tail(s)
                if !part_one {
                    print!("");
                }
                let mut t_hor_mvt = hor_mvt;
                let mut t_vert_mvt = vert_mvt;
                for idx in 1..end_tail {
                    let h_cur_x = ends[idx - 1].x;
                    let h_cur_y = ends[idx - 1].y;
                    let t_cur_x = ends[idx].x;
                    let t_cur_y = ends[idx].y;

                    let x_dif = h_cur_x - t_cur_x;
                    let y_dif = h_cur_y - t_cur_y;
                    if x_dif.abs() > 1 {
                        ends[idx].x = t_cur_x + t_hor_mvt;
                        // may need to move diag
                        if y_dif != 0 {
                            let mut y_mov = y_dif;
                            if y_mov > 1 {
                                y_mov = 1;
                            }
                            if y_mov < -1 {
                                y_mov = -1;
                            }
                            ends[idx].y = t_cur_y + y_mov;
                            t_vert_mvt = y_mov;
                        } else {
                            t_vert_mvt = 0;
                        }
                    } else if y_dif.abs() > 1 {
                        ends[idx].y = t_cur_y + t_vert_mvt;
                        // may need to move diag
                        if x_dif != 0 {
                            let mut x_mov = x_dif;
                            if x_mov > 1 {
                                x_mov = 1;
                            }
                            if x_mov < -1 {
                                x_mov = -1;
                            }
                            ends[idx].x = t_cur_x + x_mov;
                            t_hor_mvt = x_mov;
                        } else {
                            t_hor_mvt = 0;
                        }
                    } else {
                        // Nothing to move
                        t_hor_mvt = 0;
                        t_vert_mvt = 0;
                    }
                    //println!("H at {}, {}", h_cur_x, h_cur_y);
                    //println!("T at {}, {}", t_cur_x, t_cur_y);
                }

                add_pos(&mut tail_poslist, ends[end_tail - 1].x, ends[end_tail - 1].y);
            }

            if !part_one && false {
                println!("{}", line);
                print_ends(&ends);
            }
    }

        tail_poslist.len().to_string()
    }
}

fn print_ends(ends: &[Point]) {
    
    let mut min_x = 0;
    let mut min_y = 0;
    let mut max_x = 5;
    let mut max_y = 5;

    let mut map: Vec<Vec<String>> = Vec::new();

    // Find bounds
    for end in ends {
        if end.x > max_x {
            max_x = end.x;
        }
        if end.y > max_y {
            max_y = end.y;
        }
        if end.x < min_x {
            min_x = end.x;
        }
        if end.y < min_y {
            min_y = end.y;
        }        
    }

    // Fill map with .
    for _ in (min_y - 1)..(max_y + 1) {
        let mut row: Vec<String> = Vec::new();
        for _ in (min_x - 1)..(max_x + 1) {
            row.push(String::from("."));
        }
        map.push(row);
    }

    // Mark position of ends
    for idx in (0..ends.len()).rev() {
        let y = (ends[idx].y - min_y).abs() as usize;
        let x = (ends[idx].x - min_x).abs() as usize;

        let mut val = &idx.to_string()[0..1];
        if idx == 10 {
            val = "0";
        }
        map[y][x] = String::from(val);
    }

    for idx in (0..map.len()).rev() {
        let row = &map[idx as usize];
        for chr in row {
            print!("{}", chr);
        }
        println!("");
    }
    println!("");
}

struct Point {
    x: i32,
    y: i32,
}

fn add_pos(tail_poslist: &mut HashMap<String, i32>, t_cur_x: i32, t_cur_y: i32) {
    let key = format!("{}|{}", t_cur_x, t_cur_y);
    tail_poslist.entry(key).or_insert(1);
}


