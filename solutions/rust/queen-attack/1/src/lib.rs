use std::ops::{RangeInclusive};

const VALID_RANGE: RangeInclusive<i32> = 0..=7;

#[derive(Debug)]
pub struct ChessPosition {
    x: i32,
    y: i32,
}

#[derive(Debug)]
pub struct Queen {
    pos: ChessPosition,
}

impl ChessPosition {
    pub fn new(rank: i32, file: i32) -> Option<Self> {
        if !VALID_RANGE.contains(&rank) || !VALID_RANGE.contains(&file) {
            return None;
        };

        Some(Self { x: file, y: rank })
    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        Self { pos: position }
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        if self.pos.x == other.pos.x {
            return true;
        }

        if self.pos.y == other.pos.y {
            return true;
        }

        let diagonal_iters =
            // First diagonal.
            (self.pos.x + 1..=7).zip((0..self.pos.y).rev())
            .chain((0..self.pos.x).rev().zip(self.pos.y + 1..=7))
            .chain((0..self.pos.x).rev().zip((0..self.pos.y).rev()))
            .chain((self.pos.x + 1..=7).zip(self.pos.y + 1..=7));

        for (x, y) in diagonal_iters {
            if x == other.pos.x && y == other.pos.y {
                return true;
            }
        }

        false
    }
}