use crate::utils::{Grid, GridNode, get_input};

pub fn part1() -> usize {
    let grid = get_grid();
    let mut count = 0;

    for (y, row) in grid.iter().enumerate() {
        for (x, height) in row.iter().enumerate() {
            let node = grid.at(x, y).unwrap();

            if node.left_iter().all(|n| *n < *height) ||
                node.right_iter().all(|n| *n < *height) ||
                node.up_iter().all(|n| *n < *height) ||
                node.down_iter().all(|n| *n < *height)
            {
                count += 1;
            }
        }
    }

    count
}

pub fn part2() -> usize {
    let grid = get_grid();
    let mut max_vis = 0;

    fn get_num_visible_trees<'a, I>(height: u8, iter: I) -> usize 
        where I: Iterator<Item=GridNode<'a, u8>>
    {
        let mut n = 0;

        for node in iter {
            n += 1;
            if *node >= height {
                break;
            }
        }

        n
    }

    for (y, row) in grid.iter().enumerate() {
        for (x, height) in row.iter().enumerate() {
            let node = grid.at(x, y).unwrap();
            
            let total_vis = get_num_visible_trees(*height, node.left_iter())
                * get_num_visible_trees(*height, node.right_iter())
                * get_num_visible_trees(*height, node.up_iter())
                * get_num_visible_trees(*height, node.down_iter());

            max_vis = max_vis.max(total_vis);
        }
    }

    max_vis
}

fn get_grid() -> Grid<u8> {
    let mut trees = vec![];

    for line in get_input(8).lines() {
        trees.push(line.chars().map(|c| (c as u8) - ('0' as u8)).collect());
    }

    Grid::new(trees)
}
