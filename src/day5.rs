use regex::Regex;
use crate::utils::get_input;

pub fn part1() -> String {
    execute(true)
}

pub fn part2() -> String {
    execute(false)
}

fn execute(is_part_1: bool) -> String {
    let mut data = get_data();

    for move_ in &data.moves {
        let len = data.stack[move_.from].len();
        let mut chars = data.stack[move_.from].split_off(len - move_.count);
        if is_part_1 {
            chars.reverse();
        }
        data.stack[move_.to].extend(chars);
    }

    data.stack.iter().map(|s| s[s.len() - 1]).collect::<String>()
}

#[derive(Debug, Default)]
struct Data {
    stack: Vec<Vec<char>>,
    moves: Vec<Move>,
}

#[derive(Debug)]
struct Move {
    count: usize,
    from: usize,
    to: usize,
}

fn get_data() -> Data {
    let mut data = Data::default();
    let input = get_input(5);
    let lines = input.lines().collect::<Vec<_>>();
    let mut i = 0;

    let stack_regex = Regex::new(r"\[\w\] ?|    ").unwrap();
    let move_regex = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();

    while !lines[i].is_empty() {
        for (index, capture) in stack_regex.captures_iter(lines[i]).enumerate() {
            match &capture[0] {
                "    " => {},
                _ => {
                    while index >= data.stack.len() {
                        data.stack.push(vec![]);
                    }

                    data.stack[index].insert(0, capture[0].chars().nth(1).unwrap());
                }
            }
        }

        i += 1;
    }

    i += 1;

    while i < lines.len() {
        let captures = move_regex.captures(lines[i]).unwrap();
        data.moves.push(Move { 
            count: captures[1].parse().unwrap(),
            from: captures[2].parse::<usize>().unwrap() - 1,
            to: captures[3].parse::<usize>().unwrap() - 1,
        });

        i += 1;
    }

    data
}
