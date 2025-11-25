use std::fmt::{Display, Formatter};
use std::iter::repeat_n;

#[derive(PartialEq, Debug)]
pub struct RollingNumber {
    roll_at: i32,
    num: i32
}

impl RollingNumber {
    pub fn new(roll_at: u32) -> Self {
        Self { roll_at: roll_at as i32, num: 0 }
    }

    pub fn add(&mut self, value: u32) {
        for i in repeat_n(1, value as usize) {
            self.num += i;

            if self.num >= self.roll_at {
                self.num = 0
            }
        }
    }

    pub fn subtract(&mut self, value: i32) {
        for i in repeat_n(1, value.unsigned_abs() as usize) {
            if self.num == 0 {
                self.num = self.roll_at
            }

            self.num -= i;
        }
    }
}

#[derive(PartialEq, Debug)]
pub struct Clock {
    clock: RollingNumber
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let mut initial_clock = RollingNumber::new(1440);
        let total_minutes = hours * 60 + minutes;

        if total_minutes > 0 {
            initial_clock.add(total_minutes as u32);
        } else {
            initial_clock.subtract(total_minutes);
        }

        Self { clock: initial_clock }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Self::new(self.clock.num / 60, self.clock.num % 60 + minutes)
    }
}

impl Display for Clock {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let hours = format!("{:02}", self.clock.num / 60);
        let minutes = format!("{:02}", self.clock.num % 60);

        write!(f, "{}:{}", hours, minutes)
    }
}