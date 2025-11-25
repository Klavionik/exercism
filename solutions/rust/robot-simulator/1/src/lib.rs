// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(PartialEq, Eq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

pub struct Robot {
    x: i32,
    y: i32,
    direction: Direction,
}

impl Robot {
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
        Self { x, y, direction: d }
    }

    #[must_use]
    pub fn turn_right(self) -> Self {
        let next_direction = match self.direction {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        };

        Self::new(self.x, self.y, next_direction)
    }

    #[must_use]
    pub fn turn_left(self) -> Self {
        let next_direction = match self.direction {
            Direction::North => Direction::West,
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
        };

        Self::new(self.x, self.y, next_direction)
    }

    #[must_use]
    pub fn advance(self) -> Self {
        let (x, y) = match self.direction {
            Direction::North => (self.x, self.y + 1),
            Direction::East => (self.x + 1, self.y),
            Direction::South => (self.x, self.y - 1),
            Direction::West => (self.x - 1, self.y),
        };

        Self::new(x, y, self.direction)
    }

    #[must_use]
    pub fn instructions(self, instructions: &str) -> Self {
        let mut next_robot = self;

        for instruction in instructions.as_bytes() {
            next_robot = match instruction {
                b'A' => next_robot.advance(),
                b'R' => next_robot.turn_right(),
                b'L' => next_robot.turn_left(),
                _ => panic!("Incorrect instruction {instruction}."),
            }
        }

        next_robot
    }

    pub fn position(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    pub fn direction(&self) -> &Direction {
        &self.direction
    }
}
