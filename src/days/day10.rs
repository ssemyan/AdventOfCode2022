use super::Day;

pub fn run_day() {
    let day: Day = Day {
        day_num: String::from("10"),
        part_1_test: String::from("13140"),
        part_1: String::from("13720"),
        part_2_test: String::from("0"),
        part_2: String::from("0"), 
    };
    day.run_tests(&run_parts);

    fn run_parts(part_one: bool, lines: &Vec<String>) -> String {
        
        let mut reg = 1;
        let mut cycle = 0;
        let mut add_val = 0;
        let mut break_cycle = 20;
        let mut signal_str = 0;
        let mut crt = String::new();
        let mut crt_break = 40;

        // process ins
        let mut cur_line = 0;
        let mut pause_ins = false;
        loop {
            cycle = cycle + 1;

            // check for value in part one
            if cycle == break_cycle && part_one {
                signal_str = signal_str + (cycle * reg);
                println!("Cycle {} reg {} strength {}", cycle, reg, signal_str);
                if cycle == 220 {
                    break;
                }
                break_cycle = break_cycle + 40;
            }

            // add cr/lf every 40 cycles for part two
            if !part_one {
                //println!("Cycle {} reg {}", cycle, reg);

                let mut pix = ".";
                let hor_pos = cycle + 40 - crt_break - 1;
                if hor_pos >= reg - 1 && hor_pos <= reg + 1 {
                    pix = "#";
                }
                crt = crt + pix;
                if cycle == crt_break {
                    println!("{}", crt);
                    crt.clear();
                    if cycle == 240 {
                        break;
                    }
                    crt_break = crt_break + 40;
                }
            }

            // Read next ins
            if !pause_ins {
                // check for end of file
                if cur_line == lines.len() {
                    break;
                }
                let line = &lines[cur_line];
                //println!("Reading {}", line);

                cur_line = cur_line + 1;

                if line != "noop" {
                    // must be addx
                    add_val = line[5..].parse::<i32>().unwrap();
                    pause_ins = true;    
                }
            } else {
                // process prev ins
                reg = reg + add_val;
                pause_ins = false;
            }
        }

        signal_str.to_string()
    }
}
