use std::sync::{Arc, RwLock};

use crate::{TrackElement, TrackElementError};

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum MainSignalState {
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

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum AdditionalSignalState {
    Zs1 = 0x01,
    Zs7 = 0x02,
    Zs8 = 0x03,
    Zs6 = 0x04,
    Zs13 = 0x05,
    #[default]
    Off = 0xFF,
}

#[derive(Debug)]
pub struct SupportedSignalStates {
    main: Vec<MainSignalState>,
    additional: Vec<AdditionalSignalState>,
    zs3: Vec<AdditionalSignalZs3Symbol>,
    zs3v: Vec<AdditionalSignalZs3Symbol>,
}

impl Default for SupportedSignalStates {
    fn default() -> Self {
        Self {
            main: vec![MainSignalState::Off],
            additional: vec![AdditionalSignalState::Off],
            zs3: vec![AdditionalSignalZs3Symbol::OFF],
            zs3v: vec![AdditionalSignalZs3Symbol::OFF],
        }
    }
}

impl SupportedSignalStates {
    pub fn new(
        main: Vec<MainSignalState>,
        additional: Vec<AdditionalSignalState>,
        zs3: Vec<AdditionalSignalZs3Symbol>,
        zs3v: Vec<AdditionalSignalZs3Symbol>,
    ) -> Self {
        Self {
            main,
            additional,
            zs3,
            zs3v,
        }
    }

    pub fn main(mut self, main: &mut Vec<MainSignalState>) -> Self {
        self.main.append(main);
        self
    }

    pub fn additional(mut self, additional: &mut Vec<AdditionalSignalState>) -> Self {
        self.additional.append(additional);
        self
    }

    pub fn zs3(mut self, zs3: &mut Vec<AdditionalSignalZs3Symbol>) -> Self {
        self.zs3.append(zs3);
        self
    }

    pub fn zs3v(mut self, zs3v: &mut Vec<AdditionalSignalZs3Symbol>) -> Self {
        self.zs3v.append(zs3v);
        self
    }

    pub fn is_signal_state_supported(&self, state: SignalState) -> bool {
        self.main.contains(&state.main)
            && self.additional.contains(&state.additional)
            && self.zs3.contains(&state.zs3)
            && self.zs3v.contains(&state.zs3v)
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum AdditionalSignalZs3Symbol {
    #[default]
    OFF = 0xFF,
    ONE = 1,
    TWO = 2,
    THREE = 3,
    FOUR = 4,
    FIVE = 5,
    SIX = 6,
    SEVEN = 7,
    EIGHT = 8,
    NINE = 9,
    TEN = 10,
    ELEVEN = 11,
    TWELVE = 12,
    THIRTEEN = 13,
    FOURTEEN = 14,
    FIFTEEN = 15,
    SIXTEEN = 16,
}

impl TryFrom<u8> for AdditionalSignalZs3Symbol {
    type Error = TrackElementError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0xFF | 0 => Ok(AdditionalSignalZs3Symbol::OFF),
            1 => Ok(AdditionalSignalZs3Symbol::ONE),
            2 => Ok(AdditionalSignalZs3Symbol::TWO),
            3 => Ok(AdditionalSignalZs3Symbol::THREE),
            4 => Ok(AdditionalSignalZs3Symbol::FOUR),
            5 => Ok(AdditionalSignalZs3Symbol::FIVE),
            6 => Ok(AdditionalSignalZs3Symbol::SIX),
            7 => Ok(AdditionalSignalZs3Symbol::SEVEN),
            8 => Ok(AdditionalSignalZs3Symbol::EIGHT),
            9 => Ok(AdditionalSignalZs3Symbol::NINE),
            10 => Ok(AdditionalSignalZs3Symbol::TEN),
            11 => Ok(AdditionalSignalZs3Symbol::ELEVEN),
            12 => Ok(AdditionalSignalZs3Symbol::TWELVE),
            13 => Ok(AdditionalSignalZs3Symbol::THIRTEEN),
            14 => Ok(AdditionalSignalZs3Symbol::FOURTEEN),
            15 => Ok(AdditionalSignalZs3Symbol::FIFTEEN),
            16 => Ok(AdditionalSignalZs3Symbol::SIXTEEN),
            _ => Err(TrackElementError::InvalidAdditionalSignalState),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SignalState {
    main: MainSignalState,
    additional: AdditionalSignalState,
    zs3: AdditionalSignalZs3Symbol,
    zs3v: AdditionalSignalZs3Symbol,
}

impl SignalState {
    pub fn new(
        main: MainSignalState,
        additional: AdditionalSignalState,
        zs3: AdditionalSignalZs3Symbol,
        zs3v: AdditionalSignalZs3Symbol,
    ) -> Self {
        Self {
            main,
            additional,
            zs3,
            zs3v,
        }
    }

    pub fn main(&self) -> MainSignalState {
        self.main
    }

    pub fn additional(&self) -> AdditionalSignalState {
        self.additional
    }

    pub fn zs3(&self) -> AdditionalSignalZs3Symbol {
        self.zs3
    }

    pub fn zs3v(&self) -> AdditionalSignalZs3Symbol {
        self.zs3v
    }
}

impl From<MainSignalState> for SignalState {
    fn from(value: MainSignalState) -> Self {
        Self {
            main: value,
            ..Default::default()
        }
    }
}

impl Default for SignalState {
    fn default() -> Self {
        Self {
            main: MainSignalState::Hp0,
            additional: Default::default(),
            zs3: Default::default(),
            zs3v: Default::default(),
        }
    }
}

#[derive(Debug)]
pub struct Signal {
    state: SignalState,
    supported_states: SupportedSignalStates,
    id: String,
    name: Option<String>,
}

impl Signal {
    pub fn new(
        state: SignalState,
        supported_states: SupportedSignalStates,
        id: String,
        name: Option<String>,
    ) -> Self {
        Self {
            state,
            supported_states,
            id,
            name,
        }
    }

    pub fn new_arc(
        state: SignalState,
        supported_states: SupportedSignalStates,
        id: String,
        name: Option<String>,
    ) -> Arc<RwLock<Self>> {
        Arc::new(RwLock::new(Self::new(state, supported_states, id, name)))
    }

    pub fn reset(&mut self) {
        self.state = SignalState::default()
    }

    pub fn name(&self) -> &str {
        self.name.as_deref().unwrap_or(self.id()).trim()
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
        if self.supported_states.is_signal_state_supported(new_state) {
            self.state = new_state;
            println!("Signal {} is now {:?}", self.id(), self.state);
            Ok(())
        } else {
            Err(TrackElementError::InvalidMainSignalState(new_state.main))
        }
    }
}
