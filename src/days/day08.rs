use super::Day;

pub fn run_day() {
    let day: Day = Day {
        day_num: String::from("08"),
        part_1_test: String::from("21"),
        part_1: String::from("1798"),
        part_2_test: String::from("8"),
        part_2: String::from("259308"),
    };
    day.run_tests(&run_parts);

    fn run_parts(part_one: bool, lines: &Vec<String>) -> String {
        let mut forest: Vec<Vec<usize>> = Vec::new();

        // Parse into forest
        for line in lines {
            let mut row: Vec<usize> = Vec::new();

            for char in line.chars() {
                let char_val: usize = char.to_string().parse().unwrap();
                row.push(char_val);
            }
            forest.push(row);
        }

        // Trees are done
        let row_count = forest.len();
        let col_count = forest[0].len();

        let mut size = 0;
        for row_num in 0..row_count {
            for col_num in 0..col_count {
                if part_one {
                    // add up visible ones
                    if row_num == 0 || row_num == (row_count - 1) // first and last row
                    || col_num == 0 || col_num == (col_count - 1) // first and last column
                    {
                        size = size + 1;
                    } else {
                        // Check for visibility
                        if is_visible(&forest, row_num, col_num) {
                            size = size + 1;
                        }
                    }
                } else {
                    let cur_score = num_visible(&forest, row_num, col_num);
                    //println!("{} {} {}", row_num, col_num, cur_score);
                    if cur_score > size {
                        size = cur_score;
                    }
                }
            }
        }

        size.to_string()
    }
}

fn is_visible(forest: &Vec<Vec<usize>>, row_num: usize, col_num: usize) -> bool {
    let row_count = forest.len();
    let col_count = forest[0].len();

    let tree_height = forest[row_num][col_num];

    // Above
    let mut visible: bool = true;
    for row_n in 0..row_num {
        if forest[row_n][col_num] >= tree_height {
            visible = false;
            break;
        }
    }
    if visible {
        return true;
    }

    // Below
    visible = true;
    for row_n in (row_num + 1)..row_count {
        if forest[row_n][col_num] >= tree_height {
            visible = false;
            break;
        }
    }
    if visible {
        return true;
    }

    // Left
    visible = true;
    for col_n in 0..col_num {
        if forest[row_num][col_n] >= tree_height {
            visible = false;
            break;
        }
    }
    if visible {
        return true;
    }

    // Right
    visible = true;
    for col_n in (col_num + 1)..col_count {
        if forest[row_num][col_n] >= tree_height {
            visible = false;
            break;
        }
    }
    if visible {
        return true;
    }

    false
}

fn num_visible(forest: &Vec<Vec<usize>>, row_num: usize, col_num: usize) -> usize {
    let row_count = forest.len();
    let col_count = forest[0].len();

    if row_num == 0 || row_num == (row_count - 1) // first and last row
    || col_num == 0 || col_num == (col_count - 1) { // first and last column
        return 0;
    }

    let tree_height = forest[row_num][col_num];
    let mut cnt_tot = 1;

    // Above score
    for row_n in (0..row_num).rev() {
        if row_n == 0 || forest[row_n][col_num] >= tree_height {
            cnt_tot = cnt_tot * (row_num - row_n);
            break;
        }
    }

    // Below
    for row_n in (row_num + 1)..row_count {
        if row_n == (row_count - 1) || forest[row_n][col_num] >= tree_height {
            cnt_tot = cnt_tot * (row_n - row_num);
            break;
        }
    }

    // Left
    for col_n in (0..col_num).rev() {
        if col_n == 0 || forest[row_num][col_n] >= tree_height {
            cnt_tot = cnt_tot * (col_num - col_n);
            break;
        }
    }

    // Right
    for col_n in (col_num + 1)..col_count {
        if col_n == (col_count - 1) || forest[row_num][col_n] >= tree_height {
            cnt_tot = cnt_tot * (col_n - col_num);
            break;
        }
    }

    cnt_tot
}
