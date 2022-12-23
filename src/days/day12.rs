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

        // set up hashtable so we know where we have been
        let mut poslist: HashMap<String, i32> = HashMap::new();

        // Start wandering, end when we get to E
        let path_cnt = find_path(&map, 0, 0, &mut poslist, b'a', -1, i32::MAX);

        path_cnt.to_string()
    }
}

// return the path lenth to end OR i32::max if invalid (using a Result or Option is a pain)
fn find_path(
    map: &Vec<Vec<u8>>,
    cur_x: usize,
    cur_y: usize,
    poslist: &mut HashMap<String, i32>,
    prev_alt: u8,
    move_cnt: i32,
    min_move_cnt: i32,
) -> i32 {
    // add the move
    let new_move_cnt = move_cnt + 1;

    // If we found a shorter path abandon this path
    if new_move_cnt > min_move_cnt {
        return i32::MAX;
    }

    // Can only go if new spot is no more than 1 char taller
    let mut alt = map[cur_y][cur_x];
    let at_end = alt == b'E';
    if at_end {
        // at the end which is an 'e'
        alt = b'z';
    }
    if alt == b'S' {
        // at the start
        alt = b'a';
    }

    // check alt diff
    if alt > prev_alt && (alt - prev_alt) > 1 {
        // New pos too high
        return i32::MAX;
    }

    // if we are done, just return
    if at_end {
        return new_move_cnt;
    }

    // record the visit
    mark_visit(cur_x, cur_y, poslist);

    // Try all directions that are not more than one above and that are unvisited
    let mut min_path = i32::MAX; // Set the min path to the max val

    // N
    if cur_y > 0 && unvisited(cur_x, cur_y - 1, &poslist) {
        let n_cost = find_path(
            map,
            cur_x,
            cur_y - 1,
            &mut poslist.clone(),
            alt,
            new_move_cnt,
            min_move_cnt,
        );
        if n_cost < min_path {
            min_path = n_cost;
        }
    }

    // S
    if cur_y < map.len() - 1 && unvisited(cur_x, cur_y + 1, &poslist) {
        let n_cost = find_path(
            map,
            cur_x,
            cur_y + 1,
            &mut poslist.clone(),
            alt,
            new_move_cnt,
            min_move_cnt,
        );
        if n_cost < min_path {
            min_path = n_cost;
        }
    }

    // E
    if cur_x < map[0].len() - 1 && unvisited(cur_x + 1, cur_y, &poslist) {
        let n_cost = find_path(
            map,
            cur_x + 1,
            cur_y,
            &mut poslist.clone(),
            alt,
            new_move_cnt,
            min_move_cnt,
        );
        if n_cost < min_path {
            min_path = n_cost;
        }
    }

    // W
    if cur_x > 0 && unvisited(cur_x - 1, cur_y, &poslist) {
        let n_cost = find_path(
            map,
            cur_x - 1,
            cur_y,
            &mut poslist.clone(),
            alt,
            new_move_cnt,
            min_move_cnt,
        );
        if n_cost < min_path {
            min_path = n_cost;
        }
    }

    min_path
}

fn mark_visit(cur_x: usize, cur_y: usize, poslist: &mut HashMap<String, i32>) {
    let key = get_key(cur_x, cur_y);
    poslist.insert(key, 1);
}

fn unvisited(cur_x: usize, cur_y: usize, poslist: &HashMap<String, i32>) -> bool {
    let key = get_key(cur_x, cur_y);
    !poslist.contains_key(&key)
}

fn get_key(cur_x: usize, cur_y: usize) -> String {
    format!("{}|{}", cur_x, cur_y)
}
