use std::collections::HashSet;
use crate::utils::{get_input, Direction, Point};

pub fn part1() -> usize {
    execute::<2>()
}

pub fn part2() -> usize {
    execute::<10>()
}

fn execute<const N: usize>() -> usize {
    let mut tail_positions = HashSet::new();
    let mut points = [Point::new(0, 0); N];

    tail_positions.insert(points[0]);

    for movement in get_movements() {
        for _ in 0..movement.amount {
            points[0] = points[0].moved(movement.direction, 1);

            for i in 1..points.len() {
                points[i] = move_tail(points[i - 1], points[i]);
            }

            tail_positions.insert(points[points.len() - 1]);
        }
    }

    tail_positions.len()
}

pub fn move_tail(head: Point, old_tail: Point) -> Point {
    if ((head.x - old_tail.x) + (head.y - old_tail.y)).abs() <= 1 {
        return old_tail
    }

    if head.x == old_tail.x || head.y == old_tail.y {
        let delta = (old_tail - head).unit();
        return head + delta;
    }

    *get_surrounding_diagonals(old_tail).iter().min_by_key(|p| {
        (p.x - head.x).abs() + (p.y - head.y).abs()
    }).unwrap()
}

fn get_surrounding_diagonals(point: Point) -> [Point; 4] {
    [
        Point::new(point.x - 1, point.y - 1),
        Point::new(point.x - 1, point.y + 1),
        Point::new(point.x + 1, point.y - 1),
        Point::new(point.x + 1, point.y + 1),
    ]
}

#[derive(Debug)]
struct Movement {
    direction: Direction, 
    amount: usize,
}

fn get_movements() -> Vec<Movement> {
    get_input(9)
        .lines()
        .map(|l| {
            let mut l = l.split_whitespace();
            Movement {
                direction: l.next().unwrap().parse().unwrap(),
                amount: l.next().unwrap().parse().unwrap(),
            }
        })
        .collect()
}
