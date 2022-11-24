use track_element::{point::PointState, signal::SignalState};

use crate::generate::GenerationError;

#[derive(Clone, Copy)]
pub enum TrackElement {
    Point,
    Signal,
}

impl TryFrom<&serde_json::Value> for TrackElement {
    type Error = GenerationError;

    fn try_from(value: &serde_json::Value) -> Result<Self, Self::Error> {
        let v = value.as_str().ok_or(GenerationError::InvalidJson)?;
        match v {
            "point" => Ok(TrackElement::Point),
            "signal" => Ok(TrackElement::Signal),
            _ => Err(GenerationError::InvalidJson),
        }
    }
}

#[derive(Clone, Copy)]
pub enum TrackElementState {
    Point(track_element::point::PointState),
    Signal(track_element::signal::SignalState),
}

impl TryFrom<&serde_json::Value> for TrackElementState {
    type Error = GenerationError;

    fn try_from(value: &serde_json::Value) -> Result<Self, Self::Error> {
        let v = value.as_str().ok_or(GenerationError::InvalidJson)?;
        match v {
            "left" => Ok(TrackElementState::Point(PointState::Left)),
            "right" => Ok(TrackElementState::Point(PointState::Right)),
            "Hp0" => Ok(TrackElementState::Signal(SignalState::Hp0)),
            "Ks1" => Ok(TrackElementState::Signal(SignalState::Ks1)),
            "Ks2" => Ok(TrackElementState::Signal(SignalState::Ks2)),
            _ => Err(GenerationError::InvalidJson),
        }
    }
}

#[derive(Clone)]
pub struct TargetState(pub String, pub TrackElement, pub TrackElementState);

impl TryFrom<&serde_json::Value> for TargetState {
    type Error = GenerationError;

    fn try_from(value: &serde_json::Value) -> Result<Self, Self::Error> {
        Ok(TargetState(
            value["uuid"]
                .as_str()
                .ok_or(GenerationError::InvalidJson)?
                .to_string(),
            TrackElement::try_from(&value["type"])?,
            TrackElementState::try_from(&value["state"])?,
        ))
    }
}

pub struct DrivewayRepr {
    pub target_state: Vec<TargetState>,
    pub start_signal_id: String,
    pub end_signal_id: String,
}

impl TryFrom<&serde_json::Value> for DrivewayRepr {
    type Error = GenerationError;

    fn try_from(value: &serde_json::Value) -> Result<Self, Self::Error> {
        let target_states_json = value.as_array().ok_or(GenerationError::InvalidJson)?;
        let target_state: Result<Vec<TargetState>, _> = target_states_json
            .iter()
            .map(TargetState::try_from)
            .collect();
        let target_state = target_state?;

        let start_signal_id = target_state
            .iter()
            .find(|TargetState(_, t, _)| matches!(t, TrackElement::Signal))
            .ok_or(GenerationError::InvalidJson)?
            .0
            .to_string();

        let end_signal_id = target_state
            .iter()
            .filter(|TargetState(_, t, _)| matches!(t, TrackElement::Signal))
            .last()
            .ok_or(GenerationError::InvalidJson)?
            .0
            .to_string();

        Ok(DrivewayRepr {
            target_state,
            start_signal_id,
            end_signal_id,
        })
    }
}
