use std::collections::HashMap;

use super::Day;

pub fn run_day() {
    let day: Day = Day {
        day_num: String::from("12"),
        part_1_test: String::from("31"),
        part_1: String::from(""),
        part_2_test: String::from(""),
        part_2: String::from(""),
    };
    day.run_tests(&run_parts);

    fn run_parts(part_one: bool, lines: &Vec<String>) -> String {
        // load map
        let mut map: Vec<Vec<u8>> = Vec::new();
        for line in lines {
            let chars: Vec<u8> = line.clone().into_bytes();
            map.push(chars);
        }

        // set up the list of points to work through
        let mut points: Vec<PPoint> = Vec::new();

        // Set up the list of known points
        let mut k_points: HashMap<String, i32> = HashMap::new();

        // add the first point and mark as visited
        points.push(PPoint {y: 0, x: 0, dist: 0});
        mark_visit(0, 0, &mut k_points, 0);

        // loop until we get to the end
        loop {
            // get the point with the lowest dist
            let mut lowest_dist: usize = 0;
            let mut cur_low_dist = i32::MAX;
            for idx in 0..points.len() {
                if points[idx].dist < cur_low_dist {
                    cur_low_dist = points[idx].dist;
                    lowest_dist = idx;
                }
            }
            let p = points.remove(lowest_dist);            

            // get the altitude
            let mut alt = map[p.y][p.x];
            if alt == b'E' {
                // at the end
                print_map(&map, &k_points);
                return p.dist.to_string();
            }

            if alt == b'S' {
                // at the start
                alt = b'a';
            }

            // Find distance of each neighbor (will be 1) 

            // N
            if p.y > 0 {
                try_add_point(&map, &mut points, PPoint{y: p.y - 1, x: p.x, dist: p.dist + 1}, &mut k_points, alt);
            }

            // S
            try_add_point(&map, &mut points, PPoint{y: p.y + 1, x: p.x, dist: p.dist + 1}, &mut k_points, alt);

            // W
            if p.x > 0 {
                try_add_point(&map, &mut points, PPoint{y: p.y, x: p.x - 1, dist: p.dist + 1}, &mut k_points, alt);
            }

            // E
            try_add_point(&map, &mut points, PPoint{y: p.y, x: p.x + 1, dist: p.dist + 1}, &mut k_points, alt);
        }
    }
}

fn print_map(map: &Vec<Vec<u8>>, k_points: &HashMap<String, i32>) {
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            let val = check_visit(x, y, k_points);
            if val.is_none() {
                print!("-...");
            } else {
                if map[y][x] == b'E' {
                    print!("- E ");
                } else {
                    print!("-{:03}", val.unwrap());
                }
            }
        }
        println!("");
    }
}

fn try_add_point(map: &Vec<Vec<u8>>, points: &mut Vec<PPoint>, p: PPoint, k_points: &mut HashMap<String, i32>, alt: u8) {
    if check_point(&map, p.x, p.y, &k_points, alt) {
        mark_visit(p.x, p.y, k_points, p.dist);
        points.push(p);
    }
}

struct PPoint {
    y: usize,
    x: usize,
    dist: i32
}

fn check_point(map: &Vec<Vec<u8>>, new_x: usize, new_y: usize, menlist: &HashMap<String, i32>, cur_alt: u8) -> bool {
    // new node must be on the map and unvisited
    if new_y < map.len() && new_x < map[0].len() && check_visit(new_x, new_y, menlist).is_none() {
        let mut new_alt = map[new_y][new_x];
        if new_alt == b'E' {
            new_alt = b'z';
        }
        // check alt diff - must be at most one higher
        if new_alt <= cur_alt || (new_alt - cur_alt) == 1 {
            return true;
        }
    }
    false
}

fn mark_visit(cur_x: usize, cur_y: usize, poslist: &mut HashMap<String, i32>, val: i32) {
    let key = get_key(cur_x, cur_y);
    poslist.insert(key, val );
}

fn check_visit(cur_x: usize, cur_y: usize, poslist: &HashMap<String, i32>) -> Option<&i32> {
    let key = get_key(cur_x, cur_y);
    poslist.get(&key)
}

fn get_key(cur_x: usize, cur_y: usize) -> String {
    format!("{}|{}", cur_x, cur_y)
}
