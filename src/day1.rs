use crate::utils::get_input;

pub fn part1() -> u32 {
    get_calories()[0]
}

pub fn part2() -> u32 {
    get_calories().iter().take(3).sum()
}

fn get_calories() -> Vec<u32> {
    let mut calories = vec![];
    let mut curr_sum = 0;

    for line in get_input(1).lines() {
        if line.is_empty() {
            calories.push(curr_sum);
            curr_sum = 0;
        } else {
            curr_sum += line.parse::<u32>().unwrap();
        }
    }

    calories.sort();
    calories.reverse();

    calories
}
