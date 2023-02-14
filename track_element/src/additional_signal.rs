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
