use super::Day;
use regex::Regex;
use std::collections::HashMap;

pub fn run_day() {
    let day: Day = Day {
        day_num: String::from("16"),
        part_1_test: String::from("1651"),
        part_1: String::from("1896"),
        part_2_test: String::from("1707"),
        part_2: String::from(""), 
    };
    day.run_tests(&run_parts);

    fn run_parts(part_one: bool, lines: &Vec<String>) -> String {
        
        // parse input
        // Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
        let mut vlvs: HashMap<String, Valve> = HashMap::new();
        let mut vlvs_w_rate: Vec<String> = Vec::new();
        let mut zero_vlvs: Vec<String> = Vec::new();

        // add the beginning
        vlvs_w_rate.push(String::from("AA"));

        let re = Regex::new(r"^Valve ([A-Z]+) has flow rate=(\d+); tunnel[s]? lead[s]? to valve[s]? ([A-Z ,]+)$").unwrap();
        for line in lines {
            let vals = re.captures(line).unwrap();
            let v_name = vals.get(1).map_or("", |m| m.as_str()).to_string(); // OMG is this ugly
            let v_rate: i32 = vals.get(2).map_or("", |m| m.as_str()).parse().unwrap();
            let v_paths: Vec<String> = vals.get(3).map_or("", |m| m.as_str()).split(", ").map(|x| x.to_string()).collect(); 
            let v_dists: HashMap<String, i32> = HashMap::new();
            vlvs.insert(v_name.clone(), Valve{v_rate, v_paths, v_dists});
            if v_rate > 0 {
                vlvs_w_rate.push(v_name.clone());
            } else if v_name != "AA" {
                zero_vlvs.push(v_name.clone());
            }
        }
        
        // find the distance between all the valves
        for vi in 0..vlvs_w_rate.len() {
            // get dist to every valve that has a flow rate
            let v_name = &vlvs_w_rate[vi];
            for vl in 0..vlvs_w_rate.len() {
                let end_v_name = &vlvs_w_rate[vl];
                if end_v_name != v_name {
                    // first see if there is a distance already the other way
                    let dist: i32;
                    if vlvs[end_v_name].v_dists.contains_key(v_name) {
                        dist = vlvs[end_v_name].v_dists[v_name];
                    } else {
                        dist = get_dist(&v_name, end_v_name, &vlvs, 0, v_name.clone());
                    }
                    let vm = vlvs.get_mut(&vlvs_w_rate[vi]).unwrap();

                    vm.v_dists.insert(end_v_name.clone(), dist);
                }
            }
        }

        // traverse all the possible paths
        let max_flow: i32;
        if part_one {
            max_flow = best_path(&vlvs, String::from("AA"), 0, "AA", 0);
        } else {
            // // get perms of path
            // let mut valve_paths: Vec<String> = Vec::new();
            // get_perms(&mut valve_paths, vlvs_w_rate, String::new(), String::new(), true);
            // println!("Found {} perms", valve_paths.len());
            // Get rid of all the zero flow valves except "AA"
            for vlv_name in zero_vlvs {
                vlvs.remove(&vlv_name);
            }

            max_flow = best_pathe(&vlvs, String::from("AA"), 0, "AA", 0, String::from("AA"), 0, "AA", 0);
        }
        max_flow.to_string()
    }
}

fn get_perms(valve_paths: &mut Vec<String>, vlvs_w_rate: Vec<String>, my_path: String, e_path: String, my_turn: bool) {
    
    if vlvs_w_rate.len() == 0 {
        // everything assigned
        let path = format!("{},{}", my_path, e_path);
        valve_paths.push(path);
    }

    // iterate where I get the next path
    let remaining_valves_len = vlvs_w_rate.len();
    for i in 0..remaining_valves_len {
        let vlv_name = vlvs_w_rate[i].clone();
        let mut new_my_path: String = String::from(&my_path);
        if my_turn {
            new_my_path = format!("{}|{}", String::from(&my_path), vlv_name);
        } 
        let mut new_e_path: String = String::from(&e_path);
        if !my_turn {
            new_e_path = format!("{}|{}", String::from(&e_path), vlv_name);
        } 
        let left_over_valves = get_except(&vlvs_w_rate, i);
        get_perms(valve_paths, left_over_valves, new_my_path, new_e_path, !my_turn);
    }
    
}

fn get_except(strings: &Vec<String>, idx: usize) -> Vec<String> {
    let mut ret: Vec<String> = Vec::new();
    for i in 0..strings.len() {
        if i != idx {
            ret.push(strings[i].clone());
        }
    }
    ret
}

fn get_dist(start_v: &String, end_v: &String, vlvs: &HashMap<String, Valve>, dist: i32, path: String) -> i32 {
    // find the distance between the valves
    if start_v == end_v {
        return dist;
    }

    let v = &vlvs[start_v];
    let mut min_path = i32::MAX;
    for i in 0..v.v_paths.len() {
        // traverse each path and take the shortest
        let new_v = &v.v_paths[i];
        if !path.contains(new_v) {
            let p_diat = get_dist(new_v, end_v, vlvs, dist + 1, format!("{}|{}", path, new_v));
            if p_diat < min_path {
                min_path = p_diat;
            }
        }
    }
    min_path
}

fn best_path(vlvs: &HashMap<String, Valve>, cur_path: String, cur_flow: i32, cur_v_name: &str, cur_min: i32) -> i32 {

    // max run time of 30 min - should only hit this if we land on 30 min
    if cur_min >= 30 {
        return cur_flow;
    }

    // traverse from the current valve (assuming we turned it on previously)
    let mut max_flow = 0;
    for v in &vlvs[cur_v_name].v_dists {
        let flow = vlvs[v.0].v_rate;
        let end_min = cur_min + v.1 + 1;
        let pot_flow = (30 - end_min) * flow;
        if !cur_path.contains(v.0) && pot_flow > 0 {
            // gotta have enough time to get there AND turn on the valve AND let it flow for at least a minute
            let path_flow = best_path(vlvs, format!("{}|{}", cur_path, v.0), cur_flow + pot_flow, &v.0, end_min);
            if path_flow > max_flow {
                max_flow = path_flow;
            }
        }            
    }
    if max_flow == 0 {
        return cur_flow;
    }
    max_flow
}

fn best_pathe(vlvs: &HashMap<String, Valve>, cur_pathe: String, cur_flowe: i32, cur_v_namee: &str, cur_mine: i32, cur_pathm: String, cur_flowm: i32, cur_v_namem: &str, cur_minm: i32) -> i32 {

    // max run time of 26 min 
    if cur_mine >= 26 && cur_minm >= 26 {
        return cur_flowe + cur_flowm;
    }

    // traverse from the current valve (assuming we turned it on previously)
    // but now we can pick two paths to try
    let mut max_flow = 0;
    for e in &vlvs[cur_v_namee].v_dists {
        if !cur_pathe.contains(e.0) && !cur_pathm.contains(e.0) {
            let flowe = vlvs[e.0].v_rate;
            let mut end_mine = cur_mine + e.1 + 1;
            let mut pot_flowe = (26 - end_mine) * flowe;
            if pot_flowe < 0 {
                // would be going past 26 min so just reset
                end_mine = 26;
                pot_flowe = 0;
            }
            for m in &vlvs[cur_v_namem].v_dists {
                if e.0 != m.0 {
                    if !cur_pathe.contains(m.0) && !cur_pathm.contains(m.0) {
                        let flowm = vlvs[m.0].v_rate;
                        let mut end_minm = cur_minm + m.1 + 1;
                        let mut pot_flowm = (26 - end_minm) * flowm;
                        if pot_flowm < 0 {
                            // would be going past 26 min so just reset
                            end_minm = 26;
                            pot_flowm = 0;
                        }
                        let path_flow = best_pathe(vlvs, format!("{}|{}", cur_pathe, e.0), cur_flowe + pot_flowe, &e.0, 
                            end_mine, format!("{}|{}", cur_pathm, m.0), cur_flowm + pot_flowm, &m.0, end_minm);

                        if path_flow > max_flow {
                            max_flow = path_flow;
                        }
                    }
                }
            }
        }
    }

    // No more paths to follow
    if max_flow == 0 {
        return cur_flowm + cur_flowe;
    }

    max_flow
}

struct Valve {
    v_rate: i32,
    v_paths: Vec<String>,
    v_dists: HashMap<String, i32>,
}
