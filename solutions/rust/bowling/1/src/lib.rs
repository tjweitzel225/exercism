#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

#[derive(Debug)]
pub struct BowlingGame {
    standing_pins: u16,
    roll_idx: usize,
    frame_idx: usize,
    frames: [[u16; 2]; 10],
    bonus_last_frame_pins: u16,
}
impl BowlingGame {
    pub fn new() -> Self {
        BowlingGame {
            standing_pins: 10,
            roll_idx: 0,
            frame_idx: 0,
            frames: [[0; 2]; 10],
            bonus_last_frame_pins: 0,
        }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        self.standing_pins = self
            .standing_pins
            .checked_sub(pins)
            .ok_or(Error::NotEnoughPinsLeft)?;
        if let Some(r) = self
            .frames
            .get_mut(self.frame_idx)
            .ok_or(Error::GameComplete)?
            .get_mut(self.roll_idx)
        {
            *r += pins
        };
        self.roll_idx += 1;
        if self.frame_idx == 9 {
            match self.roll_idx {
                1 => self.standing_pins = 10,
                2 => {
                    let last_frame = self.frames[9];
                    if matches!(last_frame, [10, _]) | matches!(last_frame, [a, b] if a + b == 10) {
                        self.standing_pins = 10;
                    } else {
                        self.frame_idx += 1
                    }
                }
                _ => {
                    if pins > self.standing_pins && !matches!(self.frames[9], [10, 10]) {
                        self.frame_idx += 1;
                        self.bonus_last_frame_pins = pins;
                        return Err(Error::NotEnoughPinsLeft);
                    }
                    dbg!(pins, &self.standing_pins);
                    self.bonus_last_frame_pins = pins;
                    self.frame_idx += 1
                }
            }
        } else if self.standing_pins == 0 || self.roll_idx == 2 {
            self.frame_idx += 1;
            self.roll_idx = 0;
            self.standing_pins = 10;
        }
        Ok(())
    }

    pub fn score(&self) -> Option<u16> {
        dbg!(self);
        if self.frame_idx != 10 {
            return None;
        }
        let mut total = 0;
        for i in 0..9 {
            total += match self.frames[i] {
                [10, 0] => {
                    let mut pts = 10;
                    let next_frame = self.frames[i + 1];
                    dbg!(next_frame, i + 1);
                    match next_frame {
                        [10, 0] => {
                            pts += 10 + self.frames.get(i + 2).map_or(0, |f| f[0]);
                        }
                        _ => pts += next_frame[0] + next_frame[1],
                    }
                    pts
                }
                [a, b] if a + b == 10 => 10 + self.frames[i + 1][0],
                [a, b] => a + b,
            }
        }

        total += self.frames[9].iter().sum::<u16>() + self.bonus_last_frame_pins;
        Some(total)
    }
}
