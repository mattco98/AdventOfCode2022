use std::collections::HashSet;
use crate::utils::{get_input, Grid, Direction};

pub fn part1() -> usize {
    execute(true)
}

pub fn part2() -> usize {
    execute(false)
}

pub fn execute(is_part_1: bool) -> usize {
    let height_map = get_height_map();
    let diff_multiplier = if is_part_1 { 1 } else { -1 };
    let first_point = if is_part_1 { height_map.start } else { height_map.end };
    let first_point = height_map.grid.at(first_point.0, first_point.1).unwrap();

    let mut steps = 1;
    let mut points_seen = HashSet::new();
    points_seen.insert(first_point);

    let mut to_process = points_seen.clone();

    loop {
        assert!(!to_process.is_empty());

        let copy = to_process;
        to_process = HashSet::new();

        for node in copy {
            for dir in vec![Direction::Up, Direction::Down, Direction::Left, Direction::Right] {
                let new_node = node.move_in_direction(dir, 1);
                if new_node.is_none() {
                    continue;
                }
                let new_node = new_node.unwrap();
                if points_seen.contains(&new_node) {
                    continue;
                }

                let diff = *new_node as isize - *node as isize;
                if diff_multiplier * diff > 1 {
                    continue;
                }

                if is_part_1 && (new_node.x(), new_node.y()) == height_map.end {
                    return steps
                } else if !is_part_1 && *new_node == 0 {
                    return steps
                }

                to_process.insert(new_node);
                points_seen.insert(new_node);
            }
        }

        steps += 1;
    }
}

struct HeightMap {
    grid: Grid<usize>,
    start: (usize, usize),
    end: (usize, usize),
}

fn get_height_map() -> HeightMap {
    let mut data = vec![];
    let mut start = (0, 0);
    let mut end = (0, 0);
    
    for (r, line) in get_input(12).lines().enumerate() {
        let mut row = vec![];

        for (c, ch) in line.chars().enumerate() {
            let ch = if ch == 'S' {
                start = (c, r);
                'a'
            } else if ch == 'E' {
                end = (c, r);
                'z'
            } else {
                ch
            };

            row.push((ch as usize) - ('a' as usize));
        }

        data.push(row);
    }

    HeightMap { grid: Grid::new(data), start, end }
}
