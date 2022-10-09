
#[derive(Clone)]
pub struct Frame {
    rolls: Vec<i32>,
    state: FrameState
}

impl Frame {
    pub fn new(rolls: Vec<i32>) -> Self {
        Self {
            rolls,
            state: FrameState::IN_PROGRESS
        }
    }

    pub fn rolls_mut(&mut self) -> &mut Vec<i32> {
        return &mut self.rolls;
    }

    pub fn rolls(&self) -> &Vec<i32> {
        return &self.rolls;
    }

    pub fn state(&self) -> FrameState {
        return self.state;
    }

    pub fn calculate_state(&mut self) {
        
        if self.rolls[0] == 10 {
            self.state = FrameState::STRIKE;
            return;
        }
        if self.rolls.iter().sum::<i32>() == 10 {
            self.state = FrameState::SPARE;
            return;
        }
        if self.rolls.len() < 2 && self.rolls.iter().sum::<i32>() < 10 {
            return;
        }
        self.state = FrameState::COMPLETED;
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum FrameState {
    IN_PROGRESS,
    COMPLETED,
    SPARE,
    STRIKE
}