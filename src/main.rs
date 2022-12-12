#![feature(iter_array_chunks)]

mod day11;
mod utils;

use std::time::Instant;
use day11::{part1, part2};

pub fn main() {
    {
        let start = Instant::now();
        let p1 = part1();
        println!("Part 1: {} ({:.1?})", p1, start.elapsed());
    }

    {
        let start = Instant::now();
        let p2 = part2();
        println!("Part 2: {} ({:.1?})", p2, start.elapsed());
    }
}
