use regex::Regex;
use crate::utils::get_input;

pub fn part1() -> u32 {
    let mut count = 0;

    for pairs in get_pairs() {
        if let [a0, a1, b0, b1, ..] = pairs.as_slice() {
            if (a0 >= b0 && a1 <= b1) || (b0 >= a0 && b1 <= a1) {
                count += 1;
            }
        }
    }

    count
}

pub fn part2() -> u32 {
    let mut count = 0;

    for pairs in get_pairs() {
        if let [a0, a1, b0, b1, ..] = pairs.as_slice() {
            if a1 >= b0 && a0 <= b1 {
                count += 1;
            }
        }
    }

    count
}

fn get_pairs() -> Vec<Vec<i32>> {
    let mut pairs = vec![];
    let re = Regex::new(r"(\d+)-(\d+),(\d+)-(\d+)").unwrap();

    for captures in re.captures_iter(&*get_input(4)) {
        let matches = captures.iter().skip(1).map(|c| {
            c.unwrap().as_str().parse::<i32>().unwrap()
        }).collect();
        pairs.push(matches);
    }

    pairs
}
