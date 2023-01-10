use std::collections::HashSet;

use super::Day;
use regex::Regex;

pub fn run_day() {
    let day: Day = Day {
        day_num: String::from("15"),
        part_1_test: String::from("0"), // 26 in test mode
        part_1: String::from("4725496"), // 6349691 in test mode
        part_2_test: String::from(""),
        part_2: String::from(""), 
    };
    day.run_tests(&run_parts);

    fn run_parts(part_one: bool, lines: &Vec<String>) -> String {

        let mut sensors: Vec<Sensor> = Vec::new();
        let re = Regex::new(r"^Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)$").unwrap();

        for line in lines {
            let vals = re.captures(line).unwrap();
            let x: i32 = vals.get(1).map_or("", |m| m.as_str()).parse().unwrap(); // OMG is this ugly
            let y: i32 = vals.get(2).map_or("", |m| m.as_str()).parse().unwrap();
            let bx: i32 = vals.get(3).map_or("", |m| m.as_str()).parse().unwrap();
            let by: i32 = vals.get(4).map_or("", |m| m.as_str()).parse().unwrap();
            sensors.push(Sensor { x: x, y: y, bx: bx, by: by });
        }

        let row_num = 2000000;

        let mut pos_list: HashSet<i32> = HashSet::new();
        for s in &sensors {

            // determine if sensor radius falls on the line
            let man_dist = (s.x - s.bx).abs() + (s.y - s.by).abs();
            if (s.y + man_dist) >= row_num && (s.y - man_dist) <= row_num {
                let x_wid = man_dist - (row_num - s.y).abs();
                //println!("Sensor at {}, {} dist {} overlapps with width {}", s.x, s.y, man_dist, x_wid);
                for idx in 0..(x_wid + 1) {
                    insert_if_empty(&mut pos_list, s.x + idx);
                    insert_if_empty(&mut pos_list, s.x - idx);
                }
            } 
        }

        // now remove the beacons
        for s in &sensors {
            if s.by == row_num {
                if pos_list.contains(&s.bx) {
                    pos_list.remove(&s.bx);
                }
            }
        }

        pos_list.len().to_string()
    }
}

fn insert_if_empty(pos_list: &mut HashSet<i32>, v: i32) {

    if !pos_list.contains(&v) {
        pos_list.insert(v);
    }
}

struct Sensor {
    x: i32,
    y: i32,
    bx: i32,
    by: i32
}