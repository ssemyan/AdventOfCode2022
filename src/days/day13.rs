use std::collections::HashMap;

use super::Day;
use regex::Regex;

pub fn run_day() {
    let day: Day = Day {
        day_num: String::from("13"),
        part_1_test: String::from("13"),
        part_1: String::from(""), // 4474 too low
        part_2_test: String::from(""),
        part_2: String::from(""), 
    };
    day.run_tests(&run_parts);

    fn run_parts(part_one: bool, lines: &Vec<String>) -> String {
        
        let mut line_num = 0;
        let re = Regex::new(r"\[([\d,]*)\]").unwrap();

        let mut sum_good_pairs = 0;
        let mut pair_num = 1;

        loop {
            if line_num > lines.len() {//} || line_num > 30 {
                break;
            }

            // first pair
            let packet1 = get_map(&lines[line_num], &re);
            let packet2 = get_map(&lines[line_num + 1], &re);
            // let p1 = lines[line_num].as_bytes();
            // let p2 = lines[line_num + 1].as_bytes();

            // Walk the pairs 
            let ret = check_packets(&packet1.map, packet1.root, 0, &packet2.map, packet2.root, 0);
            let is_good = ret.is_none() || ret.unwrap();
            println!("Pair {} is good {}", pair_num, is_good);
            if is_good {
                sum_good_pairs = sum_good_pairs + pair_num;
            }

            //let mut p1 = lines[line_num].clone();
            //let p2 = lines[line_num + 1].clone();

            // store all the sub arrays
            // let mut p1hash: HashMap<i32, Vec<i32>> = HashMap::new();
            // let mut cur_array_code = 100;

            // loop {
            //     let vals = re.captures(&p1);
            //     if vals.is_none() {
            //         break;
            //     }
            //     // find first inner most array, and add to hashmap, and replace with its key
            //     let mtch = vals.unwrap().get(1).map_or("", |m| m.as_str());
            //     let mtch_vals: Vec<String> = mtch.split(',').map(String::from).collect::<_>();
            //     let mut mtch_ints: Vec<i32> = Vec::new();
            //     for value in mtch_vals {
            //         mtch_ints.push(value.parse().unwrap());
            //     }
            //     p1hash.insert(cur_array_code, mtch_ints);

            //     let rep_str = String::from("[") + mtch + "]";
            //     p1 = p1.replacen(&rep_str, &cur_array_code.to_string(), 1);
            // }
            // second pair
            //let p2 = &lines[line_num + 1];

            // Use RegEx to expand into lists

            // do compare
            //let right_order: bool = check_order(p1, &p2);
            
            // skip blank line
            line_num = line_num + 3;
            pair_num = pair_num + 1;
        }
        
        sum_good_pairs.to_string()
    }
}

fn check_packets(map1: &HashMap<i32, Vec<i32>>, idx1: i32, place1: usize, map2: &HashMap<i32, Vec<i32>>, idx2: i32, place2: usize) -> Option<bool> {
    
    // go through the packets
    let cur_list1 = &map1[&idx1];
    let cur_list2 = &map2[&idx2];
    let mut cur_place1 = place1;
    let mut cur_place2 = place2;

    loop {

        if cur_list1.len() == 0 || cur_place1 > cur_list1.len() - 1 {
            // run out of items in list one
            break;
        }

        // if cur_list2.len() == 0 {
        //     return Some(false);
        // }

        if cur_list2.len() == 0 || cur_place2 > cur_list2.len() - 1 {
            // second packet shorter
            return Some(false);
        }

        let item1 = cur_list1[cur_place1];
        let item2 = cur_list2[cur_place2];

        // check if one or both items are a sub list
        if item1 > 99 || item2 > 99 {
            let mut sub_index1 = idx1;
            let mut sub_index2 = idx2;
            let mut sub_place1 = cur_place1;
            let mut sub_place2 = cur_place2;
            if item1 > 99 {
                sub_index1 = item1;
                sub_place1 = 0;
            }
            if item2 > 99 {
                sub_index2 = item2;
                sub_place2 = 0;
            }
            let sub_check = check_packets(map1, sub_index1, sub_place1, map2, sub_index2, sub_place2);
            if sub_check.is_some() {
                return sub_check;
            }
            // otherwise result inconclusive so keep going
        } else {

            //println!("Comparing {} and {}", item1, item2);
            if item1 < item2 {
                return Some(true);
            } 
            if item1 > item2 {
                return Some(false);
            } 
        }
        cur_place1 = cur_place1 + 1;
        cur_place2 = cur_place2 + 1;
    }

    // if we got here, lists are equal
    None
}

fn get_map(line: &String, re: &Regex) -> Packet {
    let mut p1 = line.clone();
    //let p2 = lines[line_num + 1].clone();

    // store all the sub arrays
    let mut p1hash: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut cur_array_code = 100;

    loop {
        let vals = re.captures(&p1);
        if vals.is_none() {
            break;
        }

        // find first inner most array, and add to hashmap, and replace with its key
        cur_array_code = cur_array_code + 1;
        let mtch = vals.unwrap().get(1).map_or("", |m| m.as_str());
        let mtch_vals: Vec<String> = mtch.split(',').map(String::from).collect::<_>();
        let mut mtch_ints: Vec<i32> = Vec::new();
        for value in mtch_vals {
            let val: Result<i32, _> = value.parse();
            if val.is_ok() {
                mtch_ints.push(val.unwrap());
            }
        }
        p1hash.insert(cur_array_code, mtch_ints);

        let rep_str = String::from("[") + mtch + "]";
        p1 = p1.replacen(&rep_str, &cur_array_code.to_string(), 1);
    }

    return Packet{map: p1hash, root: cur_array_code};
}

struct Packet {
    map: HashMap<i32, Vec<i32>>,
    root: i32,
}

fn check_pair(p1: &[u8], p2: &[u8]) -> bool {
    
    // start walk to find first num
    let mut p1_pos = 0;
    let mut p2_pos = 0;
    let mut p1_bcnt = 0;
    let mut p2_bcnt = 0;
    let mut p1_val_st = 0;
    let mut p2_val_st = 0;
    let mut p2_val_st = 0;
    let mut p1_val = -1;
    let mut p1_sub_cnt = 0;

    loop {
        if p1[p1_pos] == b'[' {
            // entering a vec
            p1_bcnt = p1_bcnt + 1;
        }
        else if p1[p1_pos] == b']' && p1_val_st == 0 {
            // leaving a vec
            p1_bcnt = p1_bcnt - 1;
        } 
        else if (p1[p1_pos] == b',' || p1[p1_pos] == b']') && p1_val_st != 0 {
            // end of val
            let substr = String::from_utf8_lossy(&p1[p1_val_st..p1_pos]);
            //println!("Parsing {}", substr);
            p1_val = substr.parse().unwrap();
        } else if p1[p1_pos] >= b'0' && p1[p1_pos] <= b'9'{
            // is the start of a number, mark that we found one
            if p1_val_st == 0 {
                p1_val_st = p1_pos;
                p1_sub_cnt = p1_sub_cnt + 1;
            }
        }

        // Check if we found a val
        if p1_val > -1 {
            println!("Found {} at pos {}", p1_val, p1_bcnt);
            p1_val = -1;
            p1_val_st = 0;
        }

        p1_pos = p1_pos + 1;
        if p1_pos > p1.len() - 1 {
            break;
        }
        //println!("At pos {}", p1_pos);
    }

    true
}

// fn check_order(p1: &str, p2: &str) -> bool {
    
//     // take off outer wraps if needed
//     let pair1 = remove_outer_brackets(p1);
//     let pair2 = remove_outer_brackets(p2);

//     // Get the first value from each
//     let v1 = get_first_val(&pair1);

//     true
// }

// fn get_first_val(part_str: &str) -> i32 {
    
//     let mut first_comma = 0;
//     let mut bracket_cnt = 0;

//     if part_str.as_bytes()[0] == b'[' {
//         let new_part = remove_outer_brackets(part_str);
//         return get_first_val(&new_part);
//     }
//     for idx in 1..part.len() {
//         if part[idx] == b'[' {
//             bracket_cnt = bracket_cnt + 1;
//         } else if part[idx] == b']' {
//             bracket_cnt = bracket_cnt - 1;
//         }
//         if bracket_cnt == 0 && part[idx] == b','{
            
//             let part_val = part[0..idx + 1];
//         }
//     }
//     0
// }

// fn remove_outer_brackets(part: &str) -> String {
//     if part.starts_with('[') {
//         let match_loc = find_close_idx(part.as_bytes());
//         let unwrapped = &String::from(part)[1..match_loc];
//         return String::from(unwrapped);
//     }
//     String::from(part)
// }
// fn find_close_idx(pair: &[u8]) -> usize {

//     let mut bracket_cnt = 1;
//     for idx in 1..pair.len() {
//         if pair[idx] == b'[' {
//             bracket_cnt = bracket_cnt + 1;
//         } else if pair[idx] == b']' {
//             bracket_cnt = bracket_cnt - 1;
//         }
//         if bracket_cnt == 0 {
//             return idx;
//         }
//     }

//     return 0;
// }
