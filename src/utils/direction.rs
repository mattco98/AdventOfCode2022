#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub enum Direction {
    Left,
    Up,
    Right,
    Down,
}

#[allow(dead_code)]
impl Direction {
    pub fn flipped(&self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }

    pub fn rotated_clockwise(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    pub fn rotated_counter_clockwise(&self) -> Direction {
        self.rotated_clockwise().flipped()
    }

    pub fn dx(&self) -> isize {
        match self {
            Direction::Right => 1,
            Direction::Left => -1,
            _ => 0,
        }
    }

    pub fn dy(&self) -> isize {
        match self {
            Direction::Up => -1,
            Direction::Down => 1,
            _ => 0,
        }
    }
}

impl std::str::FromStr for Direction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match &s.to_lowercase()[..] {
            "u" | "up" => Ok(Direction::Up),
            "d" | "down" => Ok(Direction::Down),
            "l" | "left" => Ok(Direction::Left),
            "r" | "right" => Ok(Direction::Right),
            _ => Err(())
        }
    }
}
