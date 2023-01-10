use std::collections::HashSet;

use super::Day;
use regex::Regex;

pub fn run_day() {
    let day: Day = Day {
        day_num: String::from("15"),
        part_1_test: String::from("26"),  
        part_1: String::from("4725496"), // 6349691 in test mode
        part_2_test: String::from("56000011"),
        part_2: String::from("12051287042458"),
    };
    day.run_tests(&run_parts);

    fn run_parts(part_one: bool, lines: &Vec<String>) -> String {
        let mut sensors: Vec<Sensor> = Vec::new();
        let re = Regex::new(
            r"^Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)$",
        )
        .unwrap();

        for line in lines {
            let vals = re.captures(line).unwrap();
            let x: i32 = vals.get(1).map_or("", |m| m.as_str()).parse().unwrap(); // OMG is this ugly
            let y: i32 = vals.get(2).map_or("", |m| m.as_str()).parse().unwrap();
            let bx: i32 = vals.get(3).map_or("", |m| m.as_str()).parse().unwrap();
            let by: i32 = vals.get(4).map_or("", |m| m.as_str()).parse().unwrap();
            let man_dist = (x - bx).abs() + (y - by).abs();

            sensors.push(Sensor { x, y, bx, by, man_dist });
        }

        if part_one {
            let mut row_num = 2000000;
            if lines[0] == "Sensor at x=2, y=18: closest beacon is at x=-2, y=15" {
                // test mode
                row_num = 10;
            }

            let mut pos_list: HashSet<i32> = HashSet::new();
            for s in &sensors {
                // determine if sensor radius falls on the line
                if (s.y + s.man_dist) >= row_num && (s.y - s.man_dist) <= row_num {
                    let x_wid = s.man_dist - (row_num - s.y).abs();
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

            return pos_list.len().to_string();

        } else {
            let mut max_xy = 4000001;
            if lines[0] == "Sensor at x=2, y=18: closest beacon is at x=-2, y=15" {
                max_xy = 21;
            }

            // go through the list of sensors 
            for x in 0..max_xy {
                let mut y = 0;
                loop {

                    let mut found = false;
                    for s in &sensors {
                        // determine if sensor radius contains point
                        let man_dist_p = (s.x - x).abs() + (s.y - y).abs();
                        if man_dist_p <= s.man_dist {
                            // point is within radius of a sensor
                            found = true;

                            // go ahead and advance y to end of sensor distance
                            y = s.y + (s.man_dist - (s.x - x).abs()) + 1;
                            break;
                        }
                    }
                    if !found {
                        println!("X {}, y {}", x, y);
                        let ret: i64 = 4000000 * (x as i64) + (y as i64);
                        return ret.to_string();
                    }

                    if y > max_xy {
                        break;
                    }
                }
            }
        }
        String::from("notfound")
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
    by: i32,
    man_dist: i32,
}
