use std::fs;

pub fn get_input(day: u8) -> String {
    fs::read_to_string(format!("./input/day{}.txt", day)).unwrap()
}
