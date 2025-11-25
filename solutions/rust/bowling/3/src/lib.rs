const MAX_FRAMES: usize = 10;
const ALL_PINS: u16 = 10;

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

#[derive(Debug, Default)]
struct CurrentFrame(Option<u16>, Option<u16>);

impl CurrentFrame {
    fn is_strike(&self) -> bool {
        self.0.is_some_and(|score| score == ALL_PINS)
    }

    pub fn is_done(&self) -> bool {
        self.is_strike() || self.0.is_some() && self.1.is_some()
    }

    pub fn add_throw(&mut self, pins: u16) {
        if self.0.is_none() {
            self.0 = Some(pins);
        } else if self.1.is_none() {
            self.1 = Some(pins);
        }
    }

    pub fn throws(&self) -> (u16, u16) {
        (self.0.unwrap_or(0), self.1.unwrap_or(0))
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

    pub fn throw_count(&self) -> usize {
        match self {
            Frame::Strike(_) => 1,
            _ => 2,
        }
    }

    pub fn throws(&self) -> Vec<u16> {
        match self {
            Frame::Strike(pins) => vec![*pins],
            Frame::Spare(pins_1, pins_2) => vec![*pins_1, *pins_2],
            Frame::Open(pins_1, pins_2) => vec![*pins_1, *pins_2],
        }
    }

    pub fn bonus_throws(&self) -> usize {
        match self {
            Frame::Strike(_) => 2,
            Frame::Spare(_, _) => 1,
            _ => 0,
        }
    }
}

pub struct BowlingGame {
    pins: u16,
    bonus_throws: Vec<u16>,
    frames: Vec<Frame>,
    curr_frame: CurrentFrame,
    bonus_throws_left: usize,
}

impl BowlingGame {
    pub fn new() -> Self {
        Self {
            pins: ALL_PINS,
            bonus_throws: vec![],
            frames: vec![],
            curr_frame: CurrentFrame::default(),
            bonus_throws_left: 0,
        }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if self.pins < pins {
            return Err(Error::NotEnoughPinsLeft);
        }

        if self.frames.len() == MAX_FRAMES {
            return if self.bonus_throws_left > 0 {
                self.bonus_throws.push(pins);
                self.pins -= pins;
                self.bonus_throws_left -= 1;

                if self.pins == 0 {
                    self.reset_pins()
                }

                Ok(())
            } else {
                Err(Error::GameComplete)
            };
        }

        self.pins -= pins;
        self.curr_frame.add_throw(pins);

        if self.curr_frame.is_done() {
            self.finish_frame();
        }

        if self.frames.len() == MAX_FRAMES {
            self.set_bonus_throws()
        }

        Ok(())
    }

    pub fn score(&self) -> Option<u16> {
        if self.frames.len() < MAX_FRAMES || self.bonus_throws_left > 0 {
            return None;
        }

        let mut total = 0;
        let mut current_throw = 0;

        for frame in &self.frames {
            current_throw += frame.throw_count();
            let bonus = self
                .throws()
                .skip(current_throw)
                .take(frame.bonus_throws())
                .sum::<u16>();

            total += frame.score() + bonus;
        }

        Some(total)
    }

    fn reset_pins(&mut self) {
        self.pins = ALL_PINS;
    }

    fn throws(&self) -> impl Iterator<Item = u16> {
        self.frames
            .iter()
            .flat_map(|frame| frame.throws())
            .chain(self.bonus_throws.clone())
    }

    fn finish_frame(&mut self) {
        let (t1, t2) = self.curr_frame.throws();
        self.frames.push(Frame::new(t1, t2));
        self.curr_frame = CurrentFrame::default();
        self.reset_pins();
    }

    fn set_bonus_throws(&mut self) {
        let last_frame = self.frames.last().unwrap();
        self.bonus_throws_left += last_frame.bonus_throws();
    }
}