use std::collections::HashMap;

use super::Day;
use regex::Regex;

pub fn run_day() {
    let day: Day = Day {
        day_num: String::from("13"),
        part_1_test: String::from("13"),
        part_1: String::from("6046"), 
        part_2_test: String::from("140"),
        part_2: String::from("21423"), 
    };
    day.run_tests(&run_parts);

    fn run_parts(part_one: bool, lines: &Vec<String>) -> String {
        
        let mut line_num = 0;
        let re = Regex::new(r"\[([\d,]*)\]").unwrap();

        let mut sum_good_pairs = 0;
        let mut pair_num = 1;

        // Save all the packets
        let mut packets: Vec<Packet> = Vec::new();

        // add the dividers
        let dpack1 = get_map(&String::from("[[2]]"), &re);
        let dpack2 = get_map(&String::from("[[6]]"), &re);
        packets.push(dpack1);
        packets.push(dpack2);

        loop {
            if line_num > lines.len() {//} || line_num > 30 {
                break;
            }

            // first pair
            let packet1 = get_map(&lines[line_num], &re);
            let packet2 = get_map(&lines[line_num + 1], &re);
            
            // Walk the pairs 
            let ret = check_packets(&packet1.map, packet1.root, 0, &packet2.map, packet2.root, 0);
            let is_good = ret.is_none() || ret.unwrap();
            if is_good {
                //println!("Pair {} is good", pair_num);
                sum_good_pairs = sum_good_pairs + pair_num;
            }

            // add for part two
            packets.push(packet1);
            packets.push(packet2);

            // skip blank line
            line_num = line_num + 3;
            pair_num = pair_num + 1;
        }
        
        if part_one {
            return sum_good_pairs.to_string();
        }

        // part two - just bubble sort it
        let mut div1_pos: i32 = -1;
        let mut cur_pos = 0;

        loop {

            // try each of the items in the vec 
            for id1 in 0..packets.len() {
                let packet1 = &packets[id1];
                let mut is_top = true;

                for id2 in 0..packets.len() {
                    if id1 != id2 {
                        let packet2 = &packets[id2];
                        let ret = check_packets(&packet1.map, packet1.root, 0, &packet2.map, packet2.root, 0);
                        let is_good = ret.is_none() || ret.unwrap();
                        if !is_good {
                            is_top = false;
                            break;
                        }
                    }
                }
                if is_top {
                    // found the top most item - see if it was a divider (0 & 1)
                    //println!("Found the top at pos {}", cur_pos);
                    cur_pos = cur_pos + 1;
                    if div1_pos == -1 && id1 < 2 {
                        div1_pos = cur_pos;
                    } else if id1 == 0 {
                        return (div1_pos * cur_pos).to_string();
                    }
                    // remove it from the list
                    packets.remove(id1);
                    break;
                }
            }
        }
    }
}

fn check_packets(map1: &HashMap<i32, Vec<i32>>, idx1: i32, place1: usize, map2: &HashMap<i32, Vec<i32>>, idx2: i32, place2: usize) -> Option<bool> {
    
    // go through the packets
    let cur_list1 = &map1[&idx1];
    let cur_list2 = &map2[&idx2];
    let mut cur_place1 = place1;
    let mut cur_place2 = place2;

    // if list one is empty before list two we are good
    if cur_list1.len() == 0 && cur_list2.len() > 0 {
        return Some(true);
    }

    // if both are empty, keep going
    if cur_list1.len() == 0 && cur_list2.len() == 0 {
        return None;
    }

    // if list 2 is empty, list one was longer so fail
    if cur_list2.len() == 0 {
        return Some(false);
    }

    loop {

        if cur_place1 > cur_list1.len() - 1 {
            // run out of items in list one
            if cur_place2 > cur_list2.len() - 1 {
                // also out of items in list two so keep going
                return None;
            }
            // otherwise list one was shorter so all good
            return Some(true);
        }

        if cur_place2 > cur_list2.len() - 1 {
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

}

fn get_map(line: &String, re: &Regex) -> Packet {
    let mut p1 = line.clone();

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
                // make individual values single item vecs
                let mut sitem: Vec<i32> = Vec::new();
                sitem.push(val.unwrap());
                p1hash.insert(cur_array_code, sitem);
                mtch_ints.push(cur_array_code);
                cur_array_code = cur_array_code + 1;
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
