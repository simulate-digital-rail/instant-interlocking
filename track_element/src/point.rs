use crate::{TrackElement, TrackElementError};

#[derive(Debug, Clone, Copy)]
pub enum PointState {
    Left,
    Right,
}

pub struct Point {
    state: PointState,
}

impl Point {
    pub fn new(state: PointState) -> Self {
        Self { state }
    }
}

impl TrackElement for Point {
    type State = PointState;


    fn state(&self) -> Self::State {
        self.state
    }

    fn set_state(&mut self, new_state: Self::State) -> Result<(), TrackElementError>{
        self.state = new_state;
        println!("Point state is now {:?}", self.state);
        Ok(())
    }
}
