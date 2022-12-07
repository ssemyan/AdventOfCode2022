use std::fs;

pub fn get_file_lines(day: &String, is_test: bool) -> Vec<String> {

    // Read file
    //let path = env::current_dir();//.expect("cant do this");
    //println!("The current directory is {}", path.unwrap().display());
    let mut test = "Test";
    if !is_test{
        test = "";
    }
    let file_name = format!("files//Day{}{}.txt", day, test);
    //println!("Loading {}", &file_name);
    let file_contents = fs::read_to_string(&file_name)
        .expect(&format!("File {} not found.", &file_name));
    //println!("Content =\n{file_contents}");

    file_contents.split("\n").map(String::from).collect::<_>()
}
