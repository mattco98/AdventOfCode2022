use std::collections::HashSet;
use crate::utils::get_input;

pub fn part1() -> usize {
    execute(4)
}

pub fn part2() -> usize {
    execute(14)
}

fn execute(marker_len: usize) -> usize {
    get_input(6).as_bytes().windows(marker_len).position(|w| HashSet::<&u8>::from_iter(w).len() == marker_len).unwrap()
}
