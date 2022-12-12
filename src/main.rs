#![feature(iter_array_chunks)]
#![allow(dead_code)]

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod day11;
mod day12;
mod utils;

use std::fmt::Display;
use std::time::Instant;

pub fn main() {
    run_day(1, day1::part1, day1::part2);
    run_day(2, day2::part1, day2::part2);
    run_day(3, day3::part1, day3::part2);
    run_day(4, day4::part1, day4::part2);
    run_day(5, day5::part1, day5::part2);
    run_day(6, day6::part1, day6::part2);
    run_day(7, day7::part1, day7::part2);
    run_day(8, day8::part1, day8::part2);
    run_day(9, day9::part1, day9::part2);
    run_day(10, day10::part1, day10::part2);
    run_day(11, day11::part1, day11::part2);
    run_day(12, day12::part1, day12::part2);
}

fn run_day<T1: Display, T2: Display, F1: Fn() -> T1, F2: Fn() -> T2>(day: usize, part1: F1, part2: F2) {
    println!("Day {}:", day);

    {
        let start = Instant::now();
        let p1 = part1();
        println!("  Part 1: {} ({:.1?})", p1, start.elapsed());
    }

    {
        let start = Instant::now();
        let p2 = part2();
        println!("  Part 2: {} ({:.1?})", p2, start.elapsed());
    }
}
