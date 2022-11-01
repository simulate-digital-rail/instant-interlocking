use crate::TrackElement;

#[derive(Debug, Clone, Copy)]
pub enum SignalState {}

#[derive(Clone, Copy)]
pub enum SignalType {}

pub struct Signal {
    state: SignalState,
    type_: SignalType,
}

impl TrackElement for Signal {
    type State = SignalState;

    fn state(&self) -> Self::State {
        self.state
    }

    fn set_state(&mut self, new_state: Self::State) {
        self.state = new_state;
        println!("Signal is now {:?}", self.state);
    }
}
