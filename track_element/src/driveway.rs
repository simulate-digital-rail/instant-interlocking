use std::cell::{RefCell, RefMut};
use std::collections::HashMap;
use std::rc::Rc;

use crate::{
    point::{Point, PointState},
    signal::{Signal, SignalState},
};
use crate::{TrackElement, TrackElementError};

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

pub struct Driveway<'a> {
    conflicting_driveways: Vec<&'a Driveway<'a>>,
    is_set: bool,
    target_state: TargetState,
}

impl<'a> Driveway<'a> {
    pub fn new(conflicting_driveways: Vec<&'a Driveway<'a>>, expected_state: TargetState) -> Self {
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
        self.conflicting_driveways.iter().any(|d| d.is_set())
    }
}

struct DrivewayManager<'a> {
    driveways: HashMap<String, &'a Driveway<'a>>,
}

impl<'a> DrivewayManager<'a> {
    pub fn new(driveways: HashMap<String, &'a Driveway>) -> Self {
        Self { driveways }
    }

    pub fn get(&self, uuid: &str) -> Option<&'a Driveway> {
        //self.driveways.get(uuid)
        todo!()
    }
}
