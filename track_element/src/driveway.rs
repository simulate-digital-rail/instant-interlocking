use std::cell::RefCell;
use std::collections::BTreeMap;
use std::iter::Iterator;
use std::rc::Rc;

use crate::{
    point::{Point, PointState},
    signal::{Signal, SignalState},
};
use crate::{TrackElement, TrackElementError};

#[derive(Debug)]
pub struct TargetState {
    points: Vec<(Rc<RefCell<Point>>, PointState)>,
    signals: Vec<(Rc<RefCell<Signal>>, SignalState)>,
}

impl TargetState {
    pub fn new(
        points: Vec<(Rc<RefCell<Point>>, PointState)>,
        signals: Vec<(Rc<RefCell<Signal>>, SignalState)>,
    ) -> Self {
        Self { points, signals }
    }

    pub fn set_state(&mut self) -> Result<(), TrackElementError> {
        for (elem, state) in &self.points {
            elem.borrow_mut().set_state(*state)?;
        }

        // Set signals and rollback in case there was a failure
        if self
            .signals
            .iter()
            .map(|(elem, state)| elem.borrow_mut().set_state(*state))
            .any(|r| r.is_err())
        {
            self.signals
                .iter()
                .for_each(|(elem, _)| elem.borrow_mut().reset())
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct Driveway {
    conflicting_driveways: Vec<Rc<RefCell<Driveway>>>,
    is_set: bool,
    target_state: TargetState,
    start_signal: Rc<RefCell<Signal>>,
    end_signal: Rc<RefCell<Signal>>,
}

impl Driveway {
    pub fn new(
        conflicting_driveways: Vec<Rc<RefCell<Driveway>>>,
        expected_state: TargetState,
        start_signal: Rc<RefCell<Signal>>,
        end_signal: Rc<RefCell<Signal>>,
    ) -> Self {
        Self {
            conflicting_driveways,
            is_set: false,
            target_state: expected_state,
            start_signal,
            end_signal,
        }
    }

    pub fn is_set(&self) -> bool {
        self.is_set
    }

    pub fn set_way(&mut self) -> Result<(), TrackElementError> {
        if self.has_conflicting_driveways() {
            Err(TrackElementError::HasConflictingDriveways)
        } else {
            self.target_state.set_state()?;
            self.is_set = true;
            Ok(())
        }
    }

    fn has_conflicting_driveways(&self) -> bool {
        self.conflicting_driveways
            .iter()
            .any(|d| d.borrow().is_set())
    }
}

pub struct DrivewayManager {
    driveways: BTreeMap<String, Rc<RefCell<Driveway>>>,
}

impl DrivewayManager {
    pub fn new(driveways: BTreeMap<String, Rc<RefCell<Driveway>>>) -> Self {
        Self { driveways }
    }

    pub fn get(&self, uuid: &str) -> Option<Rc<RefCell<Driveway>>> {
        self.driveways.get(uuid).cloned()
    }

    pub fn get_driveway_ids(&self) -> impl Iterator<Item = &String> {
        self.driveways.keys()
    }

    pub fn add(&mut self, driveway: Rc<RefCell<Driveway>>) {
        let _driveway = driveway.clone();
        let driveway_borrow = _driveway.borrow();
        let start_signal_borrow = driveway_borrow.start_signal.borrow();
        let end_signal_borrow = driveway_borrow.end_signal.borrow();

        let id = DrivewayManager::driveway_id(start_signal_borrow.id(), end_signal_borrow.id());
        self.driveways.insert(id, driveway);
    }

    pub fn set_driveway(
        &self,
        start_signal_id: &str,
        end_signal_id: &str,
    ) -> Result<(), TrackElementError> {
        let id = DrivewayManager::driveway_id(start_signal_id, end_signal_id);
        let driveway = self
            .get(&id)
            .ok_or(TrackElementError::DrivewayDoesNotExist)?;
        driveway.borrow_mut().set_way()?;
        Ok(())
    }

    fn driveway_id(a: &str, b: &str) -> String {
        format!("{a}-{b}")
    }

    pub fn update_conflicting_driveways(&mut self) {
        for (id1, driveway) in self.driveways.iter() {
            for (id2, other) in self.driveways.iter() {
                if id1 == id2 {
                    continue;
                }
                let mut driveway = driveway.borrow_mut();
                let driveway_points = &driveway.target_state.points;
                let other_points = &other.borrow().target_state.points;
                let driveway_signals = &driveway.target_state.signals;
                let other_signals = &other.borrow().target_state.signals;
                let has_conflicting_points = driveway_points.iter().any(|(e, _)| {
                    other_points
                        .iter()
                        .any(|(o, _)| e.borrow().id() == o.borrow().id())
                });
                let has_conflicting_signals = driveway_signals.iter().any(|(e, _)| {
                    other_signals
                        .iter()
                        .any(|(o, _)| e.borrow().id() == o.borrow().id())
                });
                if has_conflicting_points || has_conflicting_signals {
                    driveway.conflicting_driveways.push(other.clone());
                }
            }
        }
    }
}
