const MAX_FRAMES: usize = 10;
const ALL_PINS: u16 = 10;

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum Turn {
    Throw(u16),
    FrameBoundary,
}

impl Turn {
    pub fn score(&self) -> u16 {
        match self {
            Turn::Throw(pins) => *pins,
            Turn::FrameBoundary => 0,
        }
    }
}

#[derive(Debug, Default)]
struct CurrentFrame(Option<u16>, Option<u16>);

impl CurrentFrame {
    pub fn is_strike(&self) -> bool {
        self.0.is_some_and(|score| score == ALL_PINS)
    }

    pub fn is_full(&self) -> bool {
        self.0.is_some() && self.1.is_some()
    }

    pub fn add_throw(&mut self, pins: u16) {
        if self.0.is_none() {
            self.0 = Some(pins);
        } else if self.1.is_none() {
            self.1 = Some(pins);
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Frame {
    Strike(u16),
    Spare(u16, u16),
    Open(u16, u16),
}

impl Frame {
    pub fn new(first_throw: u16, second_throw: u16) -> Self {
        if first_throw == ALL_PINS {
            Self::Strike(first_throw)
        } else if first_throw + second_throw == ALL_PINS {
            Self::Spare(first_throw, second_throw)
        } else {
            Self::Open(first_throw, second_throw)
        }
    }

    pub fn score(&self) -> u16 {
        match self {
            Frame::Strike(pins) => *pins,
            Frame::Spare(pins_1, pins_2) => pins_1 + pins_2,
            Frame::Open(pins_1, pins_2) => pins_1 + pins_2,
        }
    }
}

pub struct BowlingGame {
    pins: u16,
    turns: Vec<Turn>,
    curr_frame: CurrentFrame,
    bonus_throws_left: u8,
}

impl BowlingGame {
    pub fn new() -> Self {
        Self {
            pins: ALL_PINS,
            turns: vec![],
            curr_frame: CurrentFrame::default(),
            bonus_throws_left: 0,
        }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if self.pins < pins {
            return Err(Error::NotEnoughPinsLeft);
        }

        if self.frame_count() == MAX_FRAMES {
            return if self.bonus_throws_left > 0 {
                self.add_throw(pins);
                self.bonus_throws_left -= 1;

                if self.pins == 0 {
                    self.reset_pins()
                }

                Ok(())
            } else {
                Err(Error::GameComplete)
            };
        }

        self.add_throw(pins);
        self.curr_frame.add_throw(pins);

        if self.curr_frame.is_strike() || self.curr_frame.is_full() {
            self.finish_frame();
        }

        if self.frame_count() == MAX_FRAMES {
            self.set_bonus_throws()
        }

        Ok(())
    }

    pub fn score(&self) -> Option<u16> {
        if self.frame_count() < MAX_FRAMES || self.bonus_throws_left > 0 {
            return None;
        }

        let mut total = 0;

        for (frame, current_turn) in self.frames() {
            let following_throws = self.turns[current_turn..]
                .iter()
                .filter(|turn| matches!(turn, Turn::Throw(_)));

            total += frame.score();

            match frame {
                Frame::Strike(_) => {
                    let next_two_throws = following_throws
                        .take(2)
                        .map(|turn| turn.score())
                        .sum::<u16>();
                    total += next_two_throws;
                }
                Frame::Spare(_, _) => {
                    let next_throw = following_throws
                        .take(1)
                        .map(|turn| turn.score())
                        .sum::<u16>();
                    total += next_throw;
                }
                _ => (),
            }
        }

        Some(total)
    }

    fn frame_count(&self) -> usize {
        self.turns
            .iter()
            .filter(|&turn| is_frame_boundary(turn))
            .count()
    }

    fn frames(&self) -> impl Iterator<Item = (Frame, usize)> {
        let last_frame_boundary = self
            .turns
            .iter()
            .rposition(is_frame_boundary)
            .unwrap();
        let mut frames =
            self.turns[..=last_frame_boundary].split(is_frame_boundary);
        let mut current_turn = 0;

        let iter = move || {
            let group = frames.next()?;

            if group.is_empty() {
                return None;
            }

            let frame = match group {
                [Turn::Throw(pins)] => {
                    current_turn += 2;
                    Frame::new(*pins, 0)
                }
                [Turn::Throw(pins), Turn::Throw(pins_2)] => {
                    current_turn += 3;
                    Frame::new(*pins, *pins_2)
                }
                _ => unreachable!(),
            };

            Some((frame, current_turn))
        };

        std::iter::from_fn(iter)
    }

    fn get_last_frame(&self) -> Option<Frame> {
        let last_frame_boundary = self
            .turns
            .iter()
            .rposition(is_frame_boundary)
            .unwrap();
        let second_last_frame_boundary = self.turns[..last_frame_boundary]
            .iter()
            .rposition(is_frame_boundary)
            .unwrap();
        let turns = &self.turns[second_last_frame_boundary + 1..last_frame_boundary];

        match turns {
            [Turn::Throw(pins)] => Some(Frame::new(*pins, 0)),
            [Turn::Throw(pins), Turn::Throw(pins_2)] => Some(Frame::new(*pins, *pins_2)),
            _ => None,
        }
    }

    fn reset_pins(&mut self) {
        self.pins = ALL_PINS;
    }

    fn add_throw(&mut self, pins: u16) {
        self.pins -= pins;
        self.turns.push(Turn::Throw(pins));
    }

    fn finish_frame(&mut self) {
        self.turns.push(Turn::FrameBoundary);
        self.curr_frame = CurrentFrame::default();
        self.reset_pins();
    }

    fn set_bonus_throws(&mut self) {
        let last_frame = self.get_last_frame().unwrap();
        let bonus_throws = match last_frame {
            Frame::Strike(_) => 2,
            Frame::Spare(_, _) => 1,
            _ => 0,
        };

        self.bonus_throws_left += bonus_throws;
    }
}

fn is_frame_boundary(turn: &Turn) -> bool {
    matches!(turn, Turn::FrameBoundary)
}
