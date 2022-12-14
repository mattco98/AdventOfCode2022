use std::collections::HashSet;
use crate::utils::{get_input, Grid, Point, Range};

pub fn part1() -> usize {
    let (mut grid, start) = get_grid();
    let mut iterations = 0;

    loop {
        let mut curr_point = grid.at_point(start).unwrap();

        'outer: loop {
            // Try to move down
            for p in vec![curr_point.moved(0, 1), curr_point.moved(-1, 1), curr_point.moved(1, 1)] {
                if let Some(p) = p {
                    if *p == Type::Air {
                        curr_point = p;
                        continue 'outer;
                    }
                } else {
                    // The y-coordinate is out of bounds
                    return iterations;
                }
            }

            // We can't move, so we've found the final resting spot
            break;
        }

        grid.set_at_point(Point::new(curr_point.x() as isize, curr_point.y() as isize), Type::Sand);
        iterations += 1;
    }
}

pub fn part2() -> usize {
    let (grid, start) = get_grid();
    let max_y = grid.y_len() as isize;
    
    let mut queue = HashSet::new();
    queue.insert(start);

    let mut visited_points = queue.clone();

    while !queue.is_empty() {
        let mut new_queue = HashSet::new();

        for point in queue.into_iter() {
            for delta in vec![(0, 1), (-1, 1), (1, 1)] {
                let new_grid_point = point + Point::new(delta.0, delta.1);

                // If the point is in the grid, we may need to skip it. If it is not in the grid, it
                // should always be considered unless it is in the floor
                if let Some(p) = grid.at(new_grid_point.x as usize, new_grid_point.y as usize) {
                    if *p != Type::Air {
                        continue;
                    }
                } else if point.y >= max_y {
                    continue;
                }
                
                // This is a valid point
                new_queue.insert(new_grid_point);
                visited_points.insert(new_grid_point);
            }
        }

        queue = new_queue;
    }

    visited_points.len()
}

#[derive(Clone, PartialEq)]
enum Type {
    Air,
    Rock,
    Sand,
}

fn get_grid() -> (Grid<Type>, Point) {
    let mut points = HashSet::new();

    let mut min_x = 500;
    let mut max_x = 500;
    let mut min_y = 0;
    let mut max_y = 0;

    let mut last_point: Option<Point>;

    for line in get_input(14).lines() {
        last_point = None;

        for part in line.split(" -> ") {
            let mut coordinates = part.split(",");
            let x = coordinates.next().unwrap().parse::<u32>().unwrap();
            let y = coordinates.next().unwrap().parse::<u32>().unwrap();

            min_x = min_x.min(x);
            max_x = max_x.max(x);
            min_y = min_y.min(y);
            max_y = max_y.max(y);

            if let Some(p) = last_point {
                for cy in Range::inclusive(p.y, y as isize) {
                    for cx in Range::inclusive(p.x, x as isize) {
                        points.insert(Point::new(cx, cy));
                    }
                }
            }

            last_point = Some(Point::new(x as isize, y as isize));
        }

        assert!(matches!(last_point, Some(_)));
    }

    let width = max_x - min_x + 1;
    let height = max_y - min_y + 1;

    let mut grid_rows = vec![];
    for y in 0..height {
        let mut row = vec![];
        for x in 0..width {
            if points.contains(&Point::new((x + min_x) as isize, (y + min_y) as isize)) {
                row.push(Type::Rock);
            } else {
                row.push(Type::Air);
            }
        }
        grid_rows.push(row);
    }

    (Grid::new(grid_rows), Point::new(500 - min_x as isize, 0))
}
