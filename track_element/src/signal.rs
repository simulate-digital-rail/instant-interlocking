use crate::{TrackElement, TrackElementError};

// TODO: add real KS-Signal states
#[derive(Debug, Clone, Copy)]
pub enum SignalState {
    Ks1,
    Ks2,
    Hp0,
}

#[derive(Clone, Copy)]
pub enum SignalType {}

pub struct Signal {
    state: SignalState,
    type_: SignalType,
}

impl Signal {
    pub fn new(state: SignalState, type_: SignalType) -> Self {
        Self { state, type_ }
    }
    pub fn reset(&mut self){
        self.state = SignalState::Hp0
    }
}

impl TrackElement for Signal {
    type State = SignalState;

    fn state(&self) -> Self::State {
        self.state
    }

    fn set_state(&mut self, new_state: Self::State) -> Result<(), TrackElementError>{
        self.state = new_state;
        println!("Signal is now {:?}", self.state);
        Ok(())
    }
}
