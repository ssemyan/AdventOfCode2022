use super::Day;
use regex::Regex;
use std::collections::HashMap;

pub fn run_day() {
    let day: Day = Day {
        day_num: String::from("16"),
        part_1_test: String::from("1651"),
        part_1: String::from("1896"),
        part_2_test: String::from(""),
        part_2: String::from(""), 
    };
    day.run_tests(&run_parts);

    fn run_parts(part_one: bool, lines: &Vec<String>) -> String {
        
        // parse input
        // Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
        let mut vlvs: HashMap<String, Valve> = HashMap::new();
        let mut vlvs_w_rate: Vec<String> = Vec::new();

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

        // traverse all paths but use menomization to save prev paths
        let max_flow = best_path(&vlvs, String::from("AA"), 0, "AA", 0);
        max_flow.to_string()
    }
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

struct Valve {
    v_rate: i32,
    v_paths: Vec<String>,
    v_dists: HashMap<String, i32>,
}
