use std::collections::BTreeMap;
use std::iter::Iterator;
use std::sync::Arc;
use std::sync::RwLock;

use crate::{
    point::{Point, PointState},
    signal::{Signal, SignalState},
    vacancy_section::{VacancySection, VacancySectionState},
};
use crate::{TrackElement, TrackElementError};

#[derive(Debug, Clone)]
pub struct DrivewayState {
    points: Vec<(Arc<RwLock<Point>>, PointState)>,
    signals: Vec<(Arc<RwLock<Signal>>, SignalState)>,
    vacancy_sections: Vec<(Arc<RwLock<VacancySection>>, VacancySectionState)>,
}

impl DrivewayState {
    pub fn new(
        points: Vec<(Arc<RwLock<Point>>, PointState)>,
        signals: Vec<(Arc<RwLock<Signal>>, SignalState)>,
        vacancy_sections: Vec<(Arc<RwLock<VacancySection>>, VacancySectionState)>,
    ) -> Self {
        Self {
            points,
            signals,
            vacancy_sections,
        }
    }

    pub fn set_state(&mut self) -> Result<(), TrackElementError> {
        for (elem, state) in &self.points {
            elem.write().unwrap().set_state(*state)?;
        }

        // Set signals and rollback in case there was a failure
        if self
            .signals
            .iter()
            .map(|(elem, state)| elem.write().unwrap().set_state(*state))
            .any(|r| r.is_err())
        {
            self.signals
                .iter()
                .for_each(|(elem, _)| elem.write().unwrap().reset())
        }

        for (section, state) in &self.vacancy_sections {
            section.write().unwrap().set_state(*state)?;
        }

        Ok(())
    }

    fn join(mut self, mut other: DrivewayState) -> Self {
        self.points.append(&mut other.points);
        self.signals.append(&mut other.signals);
        self.vacancy_sections.append(&mut other.vacancy_sections);
        self
    }

    pub fn points(&self) -> &[(Arc<RwLock<Point>>, PointState)] {
        self.points.as_ref()
    }

    pub fn signals(&self) -> &[(Arc<RwLock<Signal>>, SignalState)] {
        self.signals.as_ref()
    }

    pub fn vacancy_sections(&self) -> &[(Arc<RwLock<VacancySection>>, VacancySectionState)] {
        self.vacancy_sections.as_ref()
    }
}

impl PartialEq for DrivewayState {
    fn eq(&self, other: &Self) -> bool {
        for ((point_a, _), (point_b, _)) in self.points.iter().zip(other.points.clone()) {
            let a = point_a.read().unwrap();
            let b = point_b.read().unwrap();
            if a.id() != b.id() {
                return false;
            }
        }
        true
    }
}

#[derive(Debug)]
pub struct Driveway {
    conflicting_driveways: Vec<Arc<RwLock<Driveway>>>,
    is_set: bool,
    target_state: DrivewayState,
    start_signal: Arc<RwLock<Signal>>,
    end_signal: Arc<RwLock<Signal>>,
}

impl Driveway {
    pub fn new(
        conflicting_driveways: Vec<Arc<RwLock<Driveway>>>,
        expected_state: DrivewayState,
        start_signal: Arc<RwLock<Signal>>,
        end_signal: Arc<RwLock<Signal>>,
    ) -> Self {
        Self {
            conflicting_driveways,
            is_set: false,
            target_state: expected_state,
            start_signal,
            end_signal,
        }
    }

    pub fn id(&self) -> String {
        format!(
            "{}-{}",
            self.start_signal.read().unwrap().id(),
            self.end_signal.read().unwrap().id()
        )
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

    pub fn state(&self) -> DrivewayState {
        let mut signals: Vec<_> = self
            .target_state
            .signals
            .iter()
            .map(|(s, _)| (s.clone(), s.read().unwrap().state()))
            .collect();
        signals.push((
            self.end_signal.clone(),
            self.end_signal.read().unwrap().state(),
        ));
        for (vacancy_section, _) in &self.target_state.vacancy_sections {
            for sig in vacancy_section.read().unwrap().previous_signals() {
                if !signals
                    .iter()
                    .any(|(s, _)| s.read().unwrap().id() == sig.read().unwrap().id())
                {
                    signals.push((sig.clone(), sig.read().unwrap().state()));
                }
            }
        }

        let vacancy_sections: Vec<_> = self
            .target_state
            .vacancy_sections
            .iter()
            .map(|(s, _)| (s.clone(), s.read().unwrap().state()))
            .collect();

        let points: Vec<_> = self
            .target_state
            .points
            .iter()
            .map(|(s, _)| (s.clone(), s.read().unwrap().state()))
            .collect();

        DrivewayState::new(points, signals, vacancy_sections)
    }

    fn has_conflicting_driveways(&self) -> bool {
        self.conflicting_driveways
            .iter()
            .any(|d| d.read().unwrap().is_set())
    }

    pub fn set_conflicting_driveways(&mut self, driveways: &mut Vec<Arc<RwLock<Driveway>>>) {
        self.conflicting_driveways.append(driveways);
    }
}

pub struct DrivewayManager {
    driveways: BTreeMap<String, Arc<RwLock<Driveway>>>,
}

impl DrivewayManager {
    pub fn new(driveways: BTreeMap<String, Arc<RwLock<Driveway>>>) -> Self {
        Self { driveways }
    }

    pub fn get(&self, uuid: &str) -> Option<Arc<RwLock<Driveway>>> {
        self.driveways.get(uuid).cloned()
    }

    pub fn get_driveway_ids(&self) -> Vec<String> {
        self.driveways
            .values()
            .map(|dw| {
                let dw = dw.read().unwrap();
                let start_signal = dw.start_signal.read().unwrap();
                let end_signal = dw.end_signal.read().unwrap();
                DrivewayManager::driveway_id(start_signal.name(), end_signal.name())
            })
            .collect()
    }

    pub fn get_point_state(&self, _element: &str) -> Result<PointState, TrackElementError> {
        todo!()
    }

    pub fn state(&self) -> DrivewayState {
        self.driveways
            .values()
            .map(|dw| dw.read().unwrap().state())
            .reduce(|a, b| a.join(b))
            .unwrap()
    }

    pub fn add(&mut self, driveway: Arc<RwLock<Driveway>>) {
        let _driveway = driveway.clone();
        let id = _driveway.read().unwrap().id();

        self.driveways.insert(id, driveway);
    }

    pub fn set_driveway(
        &self,
        start_signal_id: &str,
        end_signal_id: &str,
    ) -> Result<(), TrackElementError> {
        let id = DrivewayManager::driveway_id(start_signal_id, end_signal_id);

        let driveway = match self.get(&id) {
            Some(driveway) => driveway,
            None => {
                let state = self.state();

                let start_signal = state
                    .signals()
                    .iter()
                    .find(|(sig, _)| sig.read().unwrap().name() == start_signal_id)
                    .map(|(sig, _)| sig)
                    .ok_or(TrackElementError::DrivewayDoesNotExist(id.to_string()))?;

                let end_signal = state
                    .signals()
                    .iter()
                    .find(|(sig, _)| sig.read().unwrap().name() == end_signal_id)
                    .map(|(sig, _)| sig)
                    .ok_or(TrackElementError::DrivewayDoesNotExist(id.to_string()))?;

                let id = DrivewayManager::driveway_id(
                    start_signal.read().unwrap().id(),
                    end_signal.read().unwrap().id(),
                );

                self.driveways
                    .get(&id)
                    .ok_or(TrackElementError::DrivewayDoesNotExist(id.to_string()))?
                    .clone()
            }
        };

        driveway.write().unwrap().set_way()?;
        Ok(())
    }

    fn driveway_id(a: &str, b: &str) -> String {
        format!("{a}-{b}")
    }

    pub fn update_conflicting_driveways(&mut self) {
        for (id1, dw) in self.driveways.iter() {
            for (id2, other) in self.driveways.iter() {
                if id1 == id2 {
                    continue;
                }
                let mut driveway = dw.write().unwrap();
                let other_arc = other.clone();
                let other = other.read().unwrap();

                let self_start = driveway.start_signal.clone();
                let self_start = self_start.read().unwrap();
                let self_end = driveway.end_signal.clone();
                let self_end = self_end.read().unwrap();
                let other_start = other.start_signal.clone();
                let other_start = other_start.read().unwrap();
                let other_end = other.end_signal.clone();
                let other_end = other_end.read().unwrap();
                let are_continuous =
                    self_start.id() == other_end.id() || other_start.id() == self_end.id();

                let driveway_points = &driveway.target_state.points;
                let other_points = &other.target_state.points;
                let driveway_signals = &driveway.target_state.signals;
                let other_signals = &other.target_state.signals;
                let has_conflicting_points = driveway_points.iter().any(|(e, _)| {
                    other_points
                        .iter()
                        .any(|(o, _)| e.read().unwrap().id() == o.read().unwrap().id())
                });
                let has_conflicting_signals = driveway_signals.iter().any(|(e, _)| {
                    other_signals.iter().any(|(o, _)| {
                        e.read().unwrap().id() == o.read().unwrap().id() && !are_continuous
                    })
                });

                if has_conflicting_points || has_conflicting_signals {
                    driveway.conflicting_driveways.push(other_arc);
                }
            }
        }
    }
}
