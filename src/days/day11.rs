use super::Day;

pub fn run_day() {
    let day: Day = Day {
        day_num: String::from("11"),
        part_1_test: String::from("10605"),
        part_1: String::from("51075"),
        part_2_test: String::from("2713310158"),
        part_2: String::from("11741456163"),
    };
    day.run_tests(&run_parts);

    fn run_parts(part_one: bool, lines: &Vec<String>) -> String {
        let mut monks: Vec<Monkey> = Vec::new();

        // Parse the input
        let mut items: Vec<String> = Vec::new();
        let mut mult: Option<i32> = None;
        let mut add: Option<i32> = None;
        let mut sqr = false;
        let mut divis = 0;
        let mut d_true = 0;
        let mut d_false = 0;

        for line in lines {
            // Monkey 0:
            //   Starting items: 66, 79
            //   Operation: new = old * 11
            //   Test: divisible by 7
            //     If true: throw to monkey 6
            //     If false: throw to monkey 7
            if line.trim() != "" {
                match &line.trim_start()[0..1] {
                    "M" => (), // New monkey
                    "S" => items = line[18..].split(", ").map(String::from).collect::<_>(),
                    "O" => {
                        let oper = &line[23..24];
                        mult = None;
                        add = None;
                        sqr = false;
                        if oper == "+" {
                            add = Some(line[25..].parse::<i32>().unwrap());
                        } else if &line[25..26] == "o" {
                            sqr = true;
                        } else {
                            mult = Some(line[25..].parse::<i32>().unwrap());
                        }
                    }
                    "T" => divis = line[21..].parse::<i32>().unwrap(),
                    "I" => {
                        if &line[7..8] == "t" {
                            d_true = line[29..].parse::<i32>().unwrap();
                        } else {
                            d_false = line[30..].parse::<i32>().unwrap();

                            // Last line of desc, build monkey
                            let i_items: Vec<usize> =
                                items.iter().map(|x| x.parse::<usize>().unwrap()).collect();
                            let monk = Monkey {
                                add: add,
                                sqr: sqr,
                                mult: mult,
                                d_false: d_false as usize,
                                d_true: d_true as usize,
                                divis: divis,
                                items: i_items,
                                num_inspect: 0,
                            };
                            monks.push(monk);
                        }
                    }
                    _ => (),
                }
            }
        }

        // Set up the mitems (a version of the item for each monkey, since each monkey only cares whether it is divisable by their number)
        let mut mitems: Vec<Vec<i32>> = Vec::new();
        let monk_len = monks.len();
        for idx in 0..monk_len {
            let item_cnt = monks[idx].items.len();
            for i_idx in 0..item_cnt {
                let mut mitem: Vec<i32> = Vec::new();
                for _ in 0..monk_len {
                    let item = monks[idx].items[i_idx] as i32;
                    mitem.push(item);
                }
                mitems.push(mitem);
                monks[idx].items[i_idx] = mitems.len() - 1; // Point the item to it's index in the vector
            }
        }

        // Do the rounds
        let mut rounds = 20;
        if !part_one {
            rounds = 10000;
        }

        for round in 0..rounds {
            //println!("Round {}", round);

            // Cycle through the monkeys
            for idx in 0..monks.len() {
                
                // Mark the number of upcoming inspections
                let insp_num = monks[idx].items.len() as i32;
                monks[idx].num_inspect = monks[idx].num_inspect + insp_num;

                // Work through each of the items
                loop {
                    if monks[idx].items.len() == 0 {
                        break;
                    }

                    // Process items in order
                    let item_index = monks[idx].items[0];
                    //println!("Monkey {} item {} ", idx, item_index);

                    for iidx in 0..monk_len {

                        let mut item = mitems[item_index][iidx];

                        if monks[idx].sqr {
                            item = item * item;
                        } else if monks[idx].add.is_some() {
                            item = item + monks[idx].add.unwrap();
                        } else {
                            item = item * monks[idx].mult.unwrap();
                        }

                        if part_one {
                            item = item / 3;
                        } else {
                            // For part two, only keep the mod for the throw test of each monkey
                            item = (item % monks[iidx].divis) + monks[iidx].divis;
                        }

                        //println!("new level {}", item);
                        mitems[item_index][iidx] = item;
                    }

                    // Do the test to see where to throw to next
                    let remainder = mitems[item_index][idx] % monks[idx].divis;
                    let mut transfer_to = monks[idx].d_true;
                    if remainder != 0 {
                        transfer_to = monks[idx].d_false;
                    }
                    //println!("Transfering {} to {}",  mitems[item_index][idx], transfer_to);

                    // Now do the transfer
                    monks[transfer_to].items.push(item_index);
                    monks[idx].items.remove(0);
                }
            }

            // if !part_one && (round == 0 || round == 19 || round == 999 || round == 9999) {
            //     println!("Round {}", round + 1);
            //     for idx in 0..monk_len {
            //         println!("Monkey {}: {} ", idx, monks[idx].num_inspect);
            //     }
            // }
        }

        let mut top_insp = 0;
        let mut sec_insp = 0;

        for monk in monks {
            if monk.num_inspect > top_insp {
                sec_insp = top_insp;
                top_insp = monk.num_inspect;
            } else if monk.num_inspect > sec_insp {
                sec_insp = monk.num_inspect;
            }
        }

        println!("Top: {} and {}", top_insp, sec_insp);

        (top_insp as i64 * sec_insp as i64).to_string()
    }

    struct Monkey {
        items: Vec<usize>,
        mult: Option<i32>,
        add: Option<i32>,
        sqr: bool,
        divis: i32,
        d_true: usize,
        d_false: usize,
        num_inspect: i32,
    }
}
