use crate::{TrackElement, TrackElementError};

// TODO: add real KS-Signal states
#[derive(Default, Debug, Clone, Copy)]
pub enum SignalState {
    #[default]
    Hp0 = 0x01,
    Hp0PlusSh1 = 0x02,
    Hp0WithDrivingIndicator = 0x03,
    Ks1 = 0x04,
    Ks1Flashing = 0x05,
    Ks1FlashingWithAdditionalLight = 0x06,
    Ks2 = 0x07,
    Ks2WithAdditionalLight = 0x08,
    Sh1 = 0x09,
    IdLight = 0x0A,
    Hp0Hv = 0xA0,
    Hp1 = 0xA1,
    Hp2 = 0xA2,
    Vr0 = 0xB0,
    Vr1 = 0xB1,
    Vr2 = 0xB2,
    Off = 0xFF,
}

#[derive(Clone, Copy, Debug)]
pub enum SignalType {
    ToDo,
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
    pub fn reset(&mut self) {
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

    fn set_state(&mut self, new_state: Self::State) -> Result<(), TrackElementError> {
        self.state = new_state;
        println!("Signal {} is now {:?}", self.id(), self.state);
        Ok(())
    }
}
