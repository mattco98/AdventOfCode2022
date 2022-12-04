#![feature(iter_array_chunks)]

mod utils;
mod day3;

use day3::{part1, part2};

pub fn main() {
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}
