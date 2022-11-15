use std::cell::{RefCell, RefMut};
use std::collections::HashMap;
use std::rc::Rc;
use uuid::Uuid;

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
}

impl Driveway {
    pub fn new(conflicting_driveways: Vec<Rc<RefCell<Driveway>>>, expected_state: TargetState) -> Self {
        Self {
            conflicting_driveways,
            is_set: false,
            target_state: expected_state,
        }
    }

    pub fn is_set(&self) -> bool {
        self.is_set
    }

    pub fn set_way(&mut self) -> Result<(), TrackElementError> {
        if self.has_conflicting_driveways() {
            Err(TrackElementError)
        } else {
            self.target_state.set_state()?;
            self.is_set = true;
            Ok(())
        }
    }

    fn has_conflicting_driveways(&self) -> bool {
        self.conflicting_driveways.iter().any(|d| d.borrow().is_set())
    }
}

// TODO: Consider moving to another crate
pub struct DrivewayManager {
    driveways: HashMap<String, Rc<RefCell<Driveway>>>,
}

impl DrivewayManager {
    pub fn new(driveways: HashMap<String, Rc<RefCell<Driveway>>>) -> Self {
        Self { driveways }
    }

    pub fn get(&self, uuid: &str) -> Option<&Rc<RefCell<Driveway>>> {
        self.driveways.get(uuid)
        
    }

    pub fn add(&mut self, driveway: Rc<RefCell<Driveway>>){
        let id = Uuid::new_v4().simple().to_string();
        self.driveways.insert(id, driveway);
    }

    pub fn update_conflicting_driveways(&mut self) {
        for (id1, driveway) in self.driveways.iter() {
            for (id2, other) in self.driveways.iter() {
                if id1 == id2 {continue;}
                let mut driveway = driveway.borrow_mut();
                let driveway_points = &driveway.target_state.points;
                let other_points = &other.borrow().target_state.points;
                let driveway_signals = &driveway.target_state.signals;
                let other_signals = &other.borrow().target_state.signals;
                let has_conflicting_points = driveway_points.iter().any(|(e, _)| {other_points.iter().any(|(o, _)| e.borrow().id() == o.borrow().id())});
                let has_conflicting_signals = driveway_signals.iter().any(|(e, _)| {other_signals.iter().any(|(o, _)| e.borrow().id() == o.borrow().id())});
                if has_conflicting_points || has_conflicting_signals {
                    driveway.conflicting_driveways.push(other.clone());
                }
            }
        }
    }
}
