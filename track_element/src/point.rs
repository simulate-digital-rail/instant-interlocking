use crate::{TrackElement, TrackElementError};

#[derive(Debug, Clone, Copy)]
pub enum PointState {
    Left,
    Right,
}

impl Default for PointState {
    fn default() -> Self {
        PointState::Left
    }
}

pub struct Point {
    state: PointState,
    id: String,
}

impl Point {
    pub fn new(state: PointState, id: String) -> Self {
        Self { state, id }
    }
}

impl TrackElement for Point {
    type State = PointState;

    fn id(&self) -> &str {
        &self.id
    }

    fn state(&self) -> Self::State {
        self.state
    }

    fn set_state(&mut self, new_state: Self::State) -> Result<(), TrackElementError>{
        self.state = new_state;
        println!("Point state is now {:?}", self.state);
        Ok(())
    }
}
