pub mod util;
mod days;

fn main() {

    for x in (1..25).rev() {
        let mut str_x = x.to_string();
        if x < 10 {
            str_x = String::from("0") + &str_x;
        }
        //println!("Trying day {}", str_x);
        if days::run_day(&str_x) {
            return;
        }
    }
}
