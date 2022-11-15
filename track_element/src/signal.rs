use crate::{TrackElement, TrackElementError};

// TODO: add real KS-Signal states
#[derive(Debug, Clone, Copy)]
pub enum SignalState {
    Ks1,
    Ks2,
    Hp0,
}

impl Default for SignalState {
    fn default() -> Self {
        SignalState::Hp0
    }
}

#[derive(Clone, Copy, Debug)]
pub enum SignalType {
    ToDo
}

#[derive(Debug)]
pub struct Signal {
    state: SignalState,
    type_: SignalType,
    id: String,
}

impl Signal {
    pub fn new(state: SignalState, type_: SignalType, id: String) -> Self {
        Self { state, type_, id }
    }
    pub fn reset(&mut self){
        self.state = SignalState::Hp0
    }
}

impl TrackElement for Signal {
    type State = SignalState;

    fn id(&self) -> &str {
        &self.id
    }

    fn state(&self) -> Self::State {
        self.state
    }

    fn set_state(&mut self, new_state: Self::State) -> Result<(), TrackElementError>{
        self.state = new_state;
        println!("Signal is now {:?}", self.state);
        Ok(())
    }
}
