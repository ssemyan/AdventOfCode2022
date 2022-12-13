use super::Day;

pub fn run_day() {
    let day: Day = Day {
        day_num: String::from("07"),
        part_1_test: String::from("95437"),
        part_1: String::from("1642503"),
        part_2_test: String::from("24933642"),
        part_2: String::from("2974"),
    };
    day.run_tests(&run_parts);

    fn run_parts(part_one: bool, lines: &Vec<String>) -> String {

        // FFS, Rust does not have an easy way to do a basic tree, so we have to do this stuff
        // thanks to https://dev.to/deciduously/no-more-tears-no-more-knots-arena-allocated-trees-in-rust-44k6
        let mut node_list: Vec<Node> = Vec::new();
        let mut cur_node_id = 0;
        let root_node = Node {parent: 0, children: Vec::new(), size: 0, name: String::from("/")};
        node_list.push(root_node);

        // Parse into dir tree, skipping first line
        for line in lines[1..].iter() {
            let chars = line.as_bytes();
            
            // operation (cd or ls)
            if chars[0] == ('$' as u8) {
                if chars[2] == ('c' as u8) {
                    // Dir change
                    if chars[5] == ('.' as u8) {
                        // Go up
                        cur_node_id = node_list[cur_node_id].parent;
                    } else {
                        // Go down
                        let new_node = Node { size: 0, children: Vec::new(), parent: cur_node_id, name: line.clone() };
                        node_list.push(new_node);
                        let new_node_idx = node_list.len() - 1;
                        node_list[cur_node_id].children.push(new_node_idx);
                        cur_node_id = new_node_idx;
                    }
                } else {
                    // Doing an LS
                }
            } else if chars[0] == ('d' as u8) {
                // Directory, ignore                
            } else {
                // file from current dir
                let parts: Vec<String> = line.split(' ').map(String::from).collect::<_>();
                let file_size: i32 = parts[0].parse().unwrap();
                update_node_size(&mut node_list, cur_node_id, file_size); 
            }
        }

        // Tree is done, add up
        let mut size = 0;
        let cur_space = 70000000 - node_list[0].size;
        let space_needed = 30000000 - cur_space;
        for node in node_list {
            println!("Node {} size {}", node.name, node.size);
            if part_one {
                if node.size <= 100000 {
                    size = size + node.size;
                }
            } else {
                // Part two
                if node.size >= space_needed {
                    if size == 0 || node.size < size {
                        size = node.size;
                    }
                }
            }
        }

        size.to_string()
      
    }

    struct Node {
        parent: usize, 
        children: Vec<usize>,
        size: i32,
        name: String
    }

    fn update_node_size(node_list: &mut Vec<Node>, node_num: usize, size: i32) {
        
        node_list[node_num].size = node_list[node_num].size + size;

        if node_num != 0 {
            update_node_size(node_list, node_list[node_num].parent, size);
        }
    }
}
