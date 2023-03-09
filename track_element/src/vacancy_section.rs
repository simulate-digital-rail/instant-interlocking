use std::sync::{Arc, RwLock};

use crate::{
    signal::{MainSignalState, Signal},
    TrackElement,
};

#[derive(Debug, Clone, Copy, Default)]
pub enum VacancySectionState {
    #[default]
    Free,
    Occupied,
    CommunicationError,
    Disturbed
}

#[derive(Debug)]
pub struct VacancySection {
    id: String,
    state: VacancySectionState,
    previous_signals: Vec<Arc<RwLock<Signal>>>,
}

impl VacancySection {
    pub fn new(
        id: String,
        state: VacancySectionState,
        previous_signals: Vec<Arc<RwLock<Signal>>>,
    ) -> Self {
        Self {
            id,
            state,
            previous_signals,
        }
    }

    pub fn new_arc(
        id: String,
        state: VacancySectionState,
        previous_signals: Vec<Arc<RwLock<Signal>>>,
    ) -> Arc<RwLock<Self>> {
        Arc::new(RwLock::new(Self::new(id, state, previous_signals)))
    }

    pub fn previous_signals(&self) -> &[Arc<RwLock<Signal>>] {
        &self.previous_signals
    }
}

impl TrackElement for VacancySection {
    type State = VacancySectionState;

    fn id(&self) -> &str {
        &self.id
    }

    fn state(&self) -> Self::State {
        self.state
    }

    fn set_state(&mut self, new_state: Self::State) -> Result<(), crate::TrackElementError> {
        // TODO: Better logic, probably more like "wait until state equals expected state"
        self.state = new_state;
        for signal in &self.previous_signals {
            let mut signal = signal.write().unwrap();
            match new_state {
                VacancySectionState::Occupied => signal.set_state(MainSignalState::Hp0.into())?,
                _ => (),
            }
        }
        Ok(())
    }
}
