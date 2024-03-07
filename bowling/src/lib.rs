#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

pub struct BowlingGame {
    frames: [[u16; 3]; 10],
    current_frame: usize,
    current_roll: usize,
}

impl BowlingGame {
    pub fn new() -> Self {
        BowlingGame {
            frames: [[0; 3]; 10],
            current_frame: 0,
            current_roll: 0,
        }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if self.current_frame > 9 {
            return Err(Error::GameComplete);
        }

        if pins > 10 {
            return Err(Error::NotEnoughPinsLeft);
        }

        self.frames[self.current_frame][self.current_roll] = pins;

        if self.current_frame != 9 {
            if self.current_roll == 0 && pins == 10 {
                self.current_frame += 1;
            } else if self.current_roll == 1 {
                self.current_frame += 1;
                self.current_roll = 0;
            } else {
                self.current_roll += 1;
            }
        } else {
            if (self.current_roll == 0 && pins == 10)
                || (self.current_roll == 1 && self.frames[self.current_frame][0] + pins == 10)
            {
                self.current_roll += 1;
            } else if self.current_roll == 1 {
                self.current_frame += 1;
            } else {
                self.current_roll += 1;
                if self.current_roll > 2 {
                    self.current_frame += 1;
                }
            }
        }

        Ok(())
    }

    pub fn score(&self) -> Option<u16> {
        if self.current_frame < 10 {
            None
        } else {
            let mut score = 0;
            for i in 0..10 {
                if self.frames[i][0] == 10 {
                    // strike
                    score += 10 + self.frames[i + 1][0] + if self.frames[i + 1][1] > 0 {
                        self.frames[i + 1][1]
                    } else {
                        self.frames[i + 2][0]
                    };
                } else if self.frames[i][0] + self.frames[i][1] == 10 {
                    // spare
                    score += 10 + self.frames[i + 1][0];
                } else {
                    score += self.frames[i][0] + self.frames[i][1];
                }
            }
            Some(score)
        }
    }
}