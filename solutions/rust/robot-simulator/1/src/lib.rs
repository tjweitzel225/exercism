// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(PartialEq, Eq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}
use Direction::{East, North, South, West};

pub struct Robot {
    x: i32,
    y: i32,
    d: Direction,
}

impl Robot {
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
        Robot { x, y, d }
    }

    #[must_use]
    pub fn turn_right(self) -> Self {
        Robot {
            d: match self.d {
                North => East,
                East => South,
                South => West,
                West => North,
            },
            ..self
        }
    }

    #[must_use]
    pub fn turn_left(self) -> Self {
        Robot {
            d: match self.d {
                North => West,
                West => South,
                South => East,
                East => North,
            },
            ..self
        }
    }

    #[must_use]
    pub fn advance(self) -> Self {
        let (new_x, new_y) = match self.d {
            North => (self.x, self.y + 1),
            East => (self.x + 1, self.y),
            South => (self.x, self.y - 1),
            West => (self.x - 1, self.y),
        };
        Robot {
            x: new_x,
            y: new_y,
            ..self
        }
    }

    #[must_use]
    pub fn instructions(self, instructions: &str) -> Self {
        instructions.chars().fold(self, |robot, c| match c {
            'R' => robot.turn_right(),
            'L' => robot.turn_left(),
            'A' => robot.advance(),
            _ => unreachable!(),
        })
    }

    pub fn position(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    pub fn direction(&self) -> &Direction {
        &self.d
    }
}
