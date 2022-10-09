use crate::frame::{Frame, FrameState};

pub struct Game {
    score: i32,
    frames: Vec<Frame>
}

impl Game {
    pub fn new() -> Self{
        Self {
            score: 0,
            frames: Vec::new()
        }
    }

    fn roll(&mut self, pins: i32) {
        if self.frames.len() > 10 {
            println!("game over, can't roll anymore");
            return;
        }
        if self.frames.last().unwrap().state() != FrameState::IN_PROGRESS {
            self.frames.push(Frame::new([pins].to_vec()));
            return;
        }
        if self.frames.len() == 10 && self.frames.last().unwrap().rolls().len() < 3 {
            self.frames.last_mut().unwrap().rolls_mut().push(pins);
            return;
        }

        for frame in &mut self.frames  {
            frame.calculate_state();
        }   
    }

    fn updateScore(&mut self) {
        
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_roll() {
        
    }
}