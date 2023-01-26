use track_element::{
    additional_signal::AdditionalSignalZs3Symbol, point::PointState, signal::SignalState,
};

use crate::generate::GenerationError;

#[derive(Clone)]
pub enum TrackElement {
    Point,
    Signal,
    AdditionalSignalZs3(bool, Vec<AdditionalSignalZs3Symbol>),
}

impl TryFrom<&serde_json::Value> for TrackElement {
    type Error = GenerationError;

    fn try_from(value: &serde_json::Value) -> Result<Self, Self::Error> {
        let v = value.as_str().ok_or(GenerationError::InvalidJson)?;
        match v {
            "point" => Ok(TrackElement::Point),
            "signal" => Ok(TrackElement::Signal),
            "additional_signal_zs3" | "additional_signal_zs3v" => {
                unreachable!()
            }

            _ => Err(GenerationError::InvalidJson),
        }
    }
}

#[derive(Clone, Copy)]
pub enum TrackElementState {
    Point(track_element::point::PointState),
    Signal(track_element::signal::SignalState),
    AdditionalSignal(track_element::additional_signal::AdditionalSignalZs3Symbol),
}

impl TryFrom<&serde_json::Value> for TrackElementState {
    type Error = GenerationError;

    fn try_from(value: &serde_json::Value) -> Result<Self, Self::Error> {
        match value {
            serde_json::Value::Number(v) => {
                let n = v.as_u64().unwrap() as u8;
                let state = AdditionalSignalZs3Symbol::try_from(n)
                    .map_err(|_| GenerationError::InvalidJson)?;
                Ok(TrackElementState::AdditionalSignal(state))
            }
            serde_json::Value::String(v) => match v.as_str() {
                "left" => Ok(TrackElementState::Point(PointState::Left)),
                "right" => Ok(TrackElementState::Point(PointState::Right)),
                "Hp0" => Ok(TrackElementState::Signal(SignalState::Hp0)),
                "Ks1" => Ok(TrackElementState::Signal(SignalState::Ks1)),
                "Ks2" => Ok(TrackElementState::Signal(SignalState::Ks2)),
                _ => Err(GenerationError::InvalidJson),
            },
            _ => Err(GenerationError::InvalidJson),
        }
    }
}

#[derive(Clone)]
pub struct TargetState(pub String, pub TrackElement, pub TrackElementState);

impl TryFrom<&serde_json::Value> for TargetState {
    type Error = GenerationError;

    fn try_from(value: &serde_json::Value) -> Result<Self, Self::Error> {
        let element_type = value["type"].as_str().ok_or(GenerationError::InvalidJson)?;
        match element_type {
            "additional_signal_zs3" | "additional_signal_zs3v" => {
                let is_v = element_type.ends_with('v');
                let symbols = value["symbols"]
                    .as_array()
                    .ok_or(GenerationError::InvalidJson)?
                    .iter()
                    .map(|v| {
                        AdditionalSignalZs3Symbol::try_from(v.as_u64().unwrap() as u8).unwrap()
                    })
                    .collect();
                Ok(TargetState(
                    value["uuid"]
                        .as_str()
                        .ok_or(GenerationError::InvalidJson)?
                        .to_string(),
                    TrackElement::AdditionalSignalZs3(is_v, symbols),
                    TrackElementState::try_from(&value["state"])?,
                ))
            }
            _ => Ok(TargetState(
                value["uuid"]
                    .as_str()
                    .ok_or(GenerationError::InvalidJson)?
                    .to_string(),
                TrackElement::try_from(&value["type"])?,
                TrackElementState::try_from(&value["state"])?,
            )),
        }
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
