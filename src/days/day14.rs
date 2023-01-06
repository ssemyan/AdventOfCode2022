use std::collections::HashMap;

use super::Day;

pub fn run_day() {
    let day: Day = Day {
        day_num: String::from("14"),
        part_1_test: String::from("24"),
        part_1: String::from("638"),
        part_2_test: String::from(""),
        part_2: String::from(""),
    };
    day.run_tests(&run_parts);

    fn run_parts(part_one: bool, lines: &Vec<String>) -> String {

        // Load the points
        let mut line_points: Vec<Vec<Point>> = Vec::new();

        // mark the lowest point (highest y)
        let mut max_y = i32::MIN;

        for line in lines {
            let mut points: Vec<Point> = Vec::new();
            let parts: Vec<&str> = line.split("->").collect();
            for part in parts {
                let cords: Vec<&str> = part.split(",").collect();
                let x: i32 = cords[0].trim().parse().unwrap();
                let y: i32 = cords[1].trim().parse().unwrap();
                if y > max_y {
                    max_y = y;
                }
                points.push(Point { x, y });
            }
            line_points.push(points);
        }

        // now fill the map
        let mut map: HashMap<String, i32> = HashMap::new();

        // 1 = wall
        // 2 = sand

        for line_point in line_points {
            for idx in 1..line_point.len() {
                let p1 = &line_point[idx - 1];
                let p2 = &line_point[idx];
                
                // x & y deltas
                if p1.x != p2.x { // horizontal line
                    let mut x_itr = p1.x..p2.x + 1;
                    if p1.x > p2.x {
                        x_itr = p2.x..p1.x + 1;
                    }
                    for x in x_itr {
                        set_map(x, p1.y, &mut map, 1);
                    }    
                } else { // vertical line
                    let mut y_itr = p1.y..p2.y + 1;
                    if p1.y > p2.y {
                        y_itr = p2.y..p1.y + 1;
                    }
                    for y in y_itr {
                        set_map(p1.x, y, &mut map, 1);
                    }    
                }
            }
        }

        let mut tot_sand = 0;
        loop {
            // start dropping sand until sand falls below max y
            let sx = 500;
            let sy = 0;

            if !try_set_sand(sx, sy, &mut map, max_y) {
                break;
            }
            tot_sand = tot_sand + 1;
        }

        tot_sand.to_string()
    }
}

fn try_set_sand(sx: i32, sy: i32, map: &mut HashMap<String, i32>, max_y: i32) -> bool {
    
    if sy > max_y {
        return false; // we have fallen past the lowest wall
    }

    // check below
    if check_pos(sx, sy + 1, map) {
    
        // try diag left
        if !check_pos(sx - 1, sy + 1, map) {
            return try_set_sand(sx - 1, sy + 1, map, max_y);
        }

        // try diag right
        if !check_pos(sx + 1, sy + 1, map) {
            return try_set_sand(sx + 1, sy + 1, map, max_y);
        }

        // stop sand here
        set_map(sx, sy, map, 2);
        return true;
    }

    // otherwise keep dropping
    try_set_sand(sx, sy + 1, map, max_y)
}

struct Point {
    x: i32,
    y: i32,
}

fn get_key(x: i32, y: i32) -> String {
    format!("{}|{}", x, y)
}

fn set_map(x: i32, y: i32, map: &mut HashMap<String, i32>, val: i32) {
    let key = get_key(x, y);
    map.insert(key, val );
}

fn check_pos(x: i32, y: i32, map: &HashMap<String, i32>) -> bool {
    let key = get_key(x, y);
    map.get(&key).is_some()
}