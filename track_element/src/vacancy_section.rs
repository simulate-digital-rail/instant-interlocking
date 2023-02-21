use std::{cell::RefCell, rc::Rc};

use crate::{
    signal::{MainSignalState, Signal},
    TrackElement,
};

#[derive(Debug, Clone, Copy, Default)]
pub enum VacancySectionState {
    #[default]
    Free,
    Occupied,
}

#[derive(Debug)]
pub struct VacancySection {
    id: String,
    state: VacancySectionState,
    previous_signals: Vec<Rc<RefCell<Signal>>>,
}

impl VacancySection {
    pub fn new(
        id: String,
        state: VacancySectionState,
        previous_signals: Vec<Rc<RefCell<Signal>>>,
    ) -> Self {
        Self {
            id,
            state,
            previous_signals,
        }
    }

    pub fn new_rc(
        id: String,
        state: VacancySectionState,
        previous_signals: Vec<Rc<RefCell<Signal>>>,
    ) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self::new(id, state, previous_signals)))
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
            let mut signal = signal.borrow_mut();
            match new_state {
                VacancySectionState::Occupied => signal.set_state(MainSignalState::Hp0.into())?,
                _ => (),
            }
        }
        Ok(())
    }
}
