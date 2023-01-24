use crate::{TrackElement, TrackElementError};

#[derive(Debug)]
pub struct AdditionalSignalZs3 {
    is_v: bool,
    symbols: Vec<AdditionalSignalZs3Symbol>,
    state: AdditionalSignalZs3Symbol,
    id: String,
}

impl AdditionalSignalZs3 {
    pub fn new(
        is_v: bool,
        symbols: Vec<AdditionalSignalZs3Symbol>,
        state: AdditionalSignalZs3Symbol,
        id: String,
    ) -> Self {
        Self {
            is_v,
            id,
            symbols,
            state,
        }
    }

    pub fn is_v(&self) -> bool {
        self.is_v
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum AdditionalSignalZs3Symbol {
    #[default]
    OFF = 0,
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
            0 => Ok(AdditionalSignalZs3Symbol::OFF),
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

impl TrackElement for AdditionalSignalZs3 {
    type State = AdditionalSignalZs3Symbol;

    fn id(&self) -> &str {
        &self.id
    }

    fn state(&self) -> Self::State {
        self.state
    }

    fn set_state(&mut self, new_state: Self::State) -> Result<(), crate::TrackElementError> {
        if self.symbols.contains(&new_state) {
            self.state = new_state;
            Ok(())
        } else {
            Err(TrackElementError::InvalidAdditionalSignalState)
        }
    }
}
