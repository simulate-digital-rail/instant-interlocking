use track_element::{
    point::PointState,
    signal::{AdditionalSignalState, AdditionalSignalZs3Symbol, MainSignalState, SignalState},
};

use crate::generate::GenerationError;

#[derive(Clone)]
pub enum TrackElement {
    Point,
    Signal(
        Vec<MainSignalState>,
        Vec<AdditionalSignalZs3Symbol>,
        Vec<AdditionalSignalZs3Symbol>,
    ),
}

impl TryFrom<&serde_json::Value> for TrackElement {
    type Error = GenerationError;

    fn try_from(value: &serde_json::Value) -> Result<Self, Self::Error> {
        let v = value["type"].as_str().ok_or(GenerationError::InvalidJson)?;
        match v {
            "point" => Ok(TrackElement::Point),
            "signal" => {
                let main_states = value["supported_states"]["main"]
                    .as_array()
                    .ok_or(GenerationError::InvalidJson)?
                    .iter()
                    .map(
                        |state| match state.as_str().ok_or(GenerationError::InvalidJson)? {
                            "hp0" => Ok(MainSignalState::Hp0),
                            "hp1" => Ok(MainSignalState::Hp1),
                            "hp2" => Ok(MainSignalState::Hp2),
                            "ks1" => Ok(MainSignalState::Ks1),
                            "ks2" => Ok(MainSignalState::Ks2),
                            "sh1" => Ok(MainSignalState::Sh1),
                            _ => Err(GenerationError::InvalidJson),
                        },
                    )
                    .map(|it| it.unwrap())
                    .collect();

                let zs3_states = value["supported_states"]["zs3"]
                    .as_array()
                    .ok_or(GenerationError::InvalidJson)?
                    .iter()
                    .map(|state| {
                        AdditionalSignalZs3Symbol::try_from(
                            state.as_u64().ok_or(GenerationError::InvalidJson)? as u8,
                        )
                        .map_err(|_| GenerationError::InvalidJson)
                    })
                    .map(|it| it.unwrap())
                    .collect();

                let zs3v_states = value["supported_states"]["zs3v"]
                    .as_array()
                    .ok_or(GenerationError::InvalidJson)?
                    .iter()
                    .map(|state| {
                        AdditionalSignalZs3Symbol::try_from(
                            state.as_u64().ok_or(GenerationError::InvalidJson)? as u8,
                        )
                        .map_err(|_| GenerationError::InvalidJson)
                    })
                    .map(|it| it.unwrap())
                    .collect();

                Ok(TrackElement::Signal(main_states, zs3_states, zs3v_states))
            }
            _ => Err(GenerationError::InvalidJson),
        }
    }
}

#[derive(Clone, Copy)]
pub enum TrackElementState {
    Point(track_element::point::PointState),
    Signal(track_element::signal::SignalState),
}

fn point_state(value: &serde_json::Value) -> Result<TrackElementState, GenerationError> {
    match value.as_str().ok_or(GenerationError::InvalidJson)? {
        "left" => Ok(TrackElementState::Point(PointState::Left)),
        "right" => Ok(TrackElementState::Point(PointState::Right)),
        _ => Err(GenerationError::InvalidJson),
    }
}

fn signal_state(value: &serde_json::Value) -> Result<TrackElementState, GenerationError> {
    if !value.is_object() {
        return Err(GenerationError::InvalidJson);
    }

    let main_state = match value["main"].as_str().ok_or(GenerationError::InvalidJson)? {
        "hp0" => Ok(MainSignalState::Hp0),
        "hp1" => Ok(MainSignalState::Hp1),
        "hp2" => Ok(MainSignalState::Hp2),
        "ks1" => Ok(MainSignalState::Ks1),
        "ks2" => Ok(MainSignalState::Ks2),
        "sh1" => Ok(MainSignalState::Sh1),
        _ => Err(GenerationError::InvalidJson),
    }?;

    let additional_state = match value["additional"]
        .as_str()
        .ok_or(GenerationError::InvalidJson)?
    {
        "Zs1" => Ok(AdditionalSignalState::Zs1),
        "Zs7" => Ok(AdditionalSignalState::Zs7),
        "Zs8" => Ok(AdditionalSignalState::Zs8),
        "Zs6" => Ok(AdditionalSignalState::Zs6),
        "Zs13" => Ok(AdditionalSignalState::Zs13),
        _ => Err(GenerationError::InvalidJson),
    }?;

    let zs3_state = AdditionalSignalZs3Symbol::try_from(
        value["zs3"].as_u64().ok_or(GenerationError::InvalidJson)? as u8,
    )
    .map_err(|_| GenerationError::InvalidJson)?;

    let zs3v_state = AdditionalSignalZs3Symbol::try_from(
        value["zs3v"].as_u64().ok_or(GenerationError::InvalidJson)? as u8,
    )
    .map_err(|_| GenerationError::InvalidJson)?;

    let state = SignalState::new(main_state, additional_state, zs3_state, zs3v_state);

    Ok(TrackElementState::Signal(state))
}

#[derive(Clone)]
pub struct TargetState(pub String, pub TrackElement, pub TrackElementState);

impl TryFrom<&serde_json::Value> for TargetState {
    type Error = GenerationError;

    fn try_from(value: &serde_json::Value) -> Result<Self, Self::Error> {
        let element_type = value["type"].as_str().ok_or(GenerationError::InvalidJson)?;
        match element_type {
            "signal" => Ok(TargetState(
                value["uuid"]
                    .as_str()
                    .ok_or(GenerationError::InvalidJson)?
                    .to_string(),
                TrackElement::try_from(value)?,
                signal_state(&value["state"])?,
            )),
            "point" => Ok(TargetState(
                value["uuid"]
                    .as_str()
                    .ok_or(GenerationError::InvalidJson)?
                    .to_string(),
                TrackElement::Point,
                point_state(&value["state"])?,
            )),
            _ => Err(GenerationError::InvalidJson),
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
            .find(|TargetState(_, t, _)| matches!(t, TrackElement::Signal(_, _, _)))
            .ok_or(GenerationError::InvalidJson)?
            .0
            .to_string();

        let end_signal_id = target_state
            .iter()
            .filter(|TargetState(_, t, _)| matches!(t, TrackElement::Signal(_, _, _)))
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
