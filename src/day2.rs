use crate::utils::get_input;

// 0 == Rock, 1 == Paper, 2 == Scissors
fn score_between(me: i32, other: i32) -> i32 {
    let implicit_score = me + 1;
    // " + 1" shifts the modulus so that 0 -> loss, 1 -> draw, 2 -> win, which is the
    // correct order
    let explicit_score = (me - other + 1).rem_euclid(3) * 3;
    implicit_score + explicit_score
}

pub fn part1() -> i32 {
    get_shapes().iter().map(|(other, me)| score_between(*me, *other)).sum()
}

pub fn part2() -> i32 {
    get_shapes().iter().map(|(other, me)| {
        // According to *me:
        //     0 -> loss
        //     1 -> draw
        //     2 -> win
        // We can simply add them together, and then add 2 to shift
        // the result to be correct
        score_between((*other + *me + 2).rem_euclid(3), *other)
    }).sum()
}

fn get_shapes() -> Vec<(i32, i32)> {
    get_input(2).lines().map(|l| {
        let v = l
            .split_whitespace()
            .map(|p| match p {
                "A" | "X" => 0,
                "B" | "Y" => 1,
                "C" | "Z" => 2,
                _ => unreachable!(),
            })
            .collect::<Vec<_>>();
        (v[0], v[1])
    }).collect()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_loss() {
        assert_eq!(score_between(0, 1), 1);
        assert_eq!(score_between(1, 2), 2);
        assert_eq!(score_between(2, 3), 3);
    }

    #[test]
    pub fn test_draw() {
        assert_eq!(score_between(0, 0), 4);
        assert_eq!(score_between(1, 1), 5);
        assert_eq!(score_between(2, 2), 6);
    }

    #[test]
    pub fn test_win() {
        assert_eq!(score_between(0, 2), 7);
        assert_eq!(score_between(1, 0), 8);
        assert_eq!(score_between(2, 1), 9);
    }
}
