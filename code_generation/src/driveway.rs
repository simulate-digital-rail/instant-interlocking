use proc_macro2::TokenStream;
use quote::quote;
use serde::Deserialize;

use crate::generate::{uuid_to_var_name, GenerationError};

pub trait Realize {
    fn realize(&self) -> TokenStream;
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct DrivewayRepr {
    pub start_signal: TrackElement,
    pub end_signal: TrackElement,
    pub states: Vec<TrackElement>,
}

impl DrivewayRepr {
    pub fn id(&self) -> String {
        format!("{}_{}", self.start_signal.id(), self.end_signal.id())
    }
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum TrackElement {
    Point {
        uuid: String,
        state: PointState,
    },

    Signal {
        uuid: String,
        name: Option<String>,
        supported_states: SupportedSignalStates,
        state: SignalState,
    },

    VacancySection {
        uuid: String,
        state: VacancySectionState,
        previous_signals: Vec<TrackElement>,
    },
}

impl TrackElement {
    pub fn id(&self) -> &str {
        match self {
            TrackElement::Point { uuid, .. } => uuid,
            TrackElement::Signal { uuid, .. } => uuid,
            TrackElement::VacancySection { uuid, .. } => uuid,
        }
    }
}

impl Realize for TrackElement {
    fn realize(&self) -> TokenStream {
        match self {
            TrackElement::Point { uuid, .. } => {
                quote! {track_element::point::Point::new_arc(track_element::point::PointState::default(), #uuid.to_string())}
            }
            TrackElement::Signal {
                uuid,
                name,
                supported_states,
                ..
            } => {
                let supported_states = supported_states.realize();
                let name = match name {
                    Some(name) => quote! { Some(#name.to_string()) },
                    None => quote! { None },
                };
                quote! {
                    track_element::signal::Signal::new_arc(track_element::signal::SignalState::default(), #supported_states, #uuid.to_string(), #name)
                }
            }
            TrackElement::VacancySection {
                uuid,
                previous_signals,
                ..
            } => {
                let prev_signals: Vec<_> = previous_signals
                    .iter()
                    .map(|signal| {
                        let var = uuid_to_var_name(signal.id());
                        quote! { #var.clone() }
                    })
                    .collect();

                quote! {
                    track_element::vacancy_section::VacancySection::new_arc(#uuid.to_string(), track_element::vacancy_section::VacancySectionState::default(), vec![#(#prev_signals),*])
                }
            }
        }
    }
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct SignalState {
    pub main: MainSignalState,
    pub zs3: Option<AdditionalSignalZs3Symbol>,
    pub zs3v: Option<AdditionalSignalZs3Symbol>,
}

impl Realize for SignalState {
    fn realize(&self) -> TokenStream {
        let main = self.main.realize();
        let zs3 = match &self.zs3 {
            Some(zs3) => zs3.realize(),
            None => AdditionalSignalZs3Symbol(0).realize(),
        };
        let zs3v = match &self.zs3v {
            Some(zs3v) => zs3v.realize(),
            None => AdditionalSignalZs3Symbol(0).realize(),
        };
        quote! {
            track_element::signal::SignalState::new(#main, track_element::signal::AdditionalSignalState::Off, #zs3, #zs3v)
        }
    }
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct SupportedSignalStates {
    pub main: Vec<MainSignalState>,
    pub zs3: Option<Vec<AdditionalSignalZs3Symbol>>,
    pub zs3v: Option<Vec<AdditionalSignalZs3Symbol>>,
}

impl Realize for SupportedSignalStates {
    fn realize(&self) -> TokenStream {
        let main = self.main.iter().map(Realize::realize);
        let zs3 = match &self.zs3 {
            Some(states) => states.iter().map(Realize::realize).collect(),
            None => vec![],
        };
        let zs3v = match &self.zs3v {
            Some(states) => states.iter().map(Realize::realize).collect(),
            None => vec![],
        };

        quote! {
            track_element::signal::SupportedSignalStates::default().main(&mut vec![#(#main),*]).zs3(&mut vec![#(#zs3),*]).zs3v(&mut vec![#(#zs3v),*])
        }
    }
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct MainSignalState(pub String);

impl TryInto<track_element::signal::MainSignalState> for &MainSignalState {
    type Error = GenerationError;

    fn try_into(self) -> Result<track_element::signal::MainSignalState, Self::Error> {
        let lower = self.0.to_ascii_lowercase();
        match lower.as_str() {
            "ks1" => Ok(track_element::signal::MainSignalState::Ks1),
            "ks2" => Ok(track_element::signal::MainSignalState::Ks2),
            "hp0" => Ok(track_element::signal::MainSignalState::Hp0),
            "hp1" => Ok(track_element::signal::MainSignalState::Hp1),
            "hp2" => Ok(track_element::signal::MainSignalState::Hp2),
            "sh1" => Ok(track_element::signal::MainSignalState::Sh1),
            e => Err(GenerationError::InvalidJson(format!(
                "Unknown main signal state `{e}`"
            ))),
        }
    }
}

impl Realize for MainSignalState {
    fn realize(&self) -> TokenStream {
        let state: track_element::signal::MainSignalState = self.try_into().unwrap();
        match state {
            track_element::signal::MainSignalState::Hp0 => {
                quote! {track_element::signal::MainSignalState::Hp0}
            }
            track_element::signal::MainSignalState::Hp0PlusSh1 => {
                quote! {track_element::signal::MainSignalState::Hp0PlusSh1}
            }
            track_element::signal::MainSignalState::Hp0WithDrivingIndicator => {
                quote! {track_element::signal::MainSignalState::Hp0WithDrivingIndicator}
            }
            track_element::signal::MainSignalState::Ks1 => {
                quote! {track_element::signal::MainSignalState::Ks1}
            }
            track_element::signal::MainSignalState::Ks1Flashing => {
                quote! {track_element::signal::MainSignalState::Ks1Flashing}
            }
            track_element::signal::MainSignalState::Ks1FlashingWithAdditionalLight => {
                quote! {track_element::signal::MainSignalState::Ks1FlashingWithAdditionalLight}
            }
            track_element::signal::MainSignalState::Ks2 => {
                quote! {track_element::signal::MainSignalState::Ks2}
            }
            track_element::signal::MainSignalState::Ks2WithAdditionalLight => {
                quote! {track_element::signal::MainSignalState::Ks2WithAdditionalLight}
            }
            track_element::signal::MainSignalState::Sh1 => {
                quote! {track_element::signal::MainSignalState::Sh1}
            }
            track_element::signal::MainSignalState::IdLight => {
                quote! {track_element::signal::MainSignalState::IdLight}
            }
            track_element::signal::MainSignalState::Hp0Hv => {
                quote! {track_element::signal::MainSignalState::Hp0Hv}
            }
            track_element::signal::MainSignalState::Hp1 => {
                quote! {track_element::signal::MainSignalState::Hp1}
            }
            track_element::signal::MainSignalState::Hp2 => {
                quote! {track_element::signal::MainSignalState::Hp2}
            }
            track_element::signal::MainSignalState::Vr0 => {
                quote! {track_element::signal::MainSignalState::Vr0}
            }
            track_element::signal::MainSignalState::Vr1 => {
                quote! {track_element::signal::MainSignalState::Vr1}
            }
            track_element::signal::MainSignalState::Vr2 => {
                quote! {track_element::signal::MainSignalState::Vr2}
            }
            track_element::signal::MainSignalState::Off => {
                quote! {track_element::signal::MainSignalState::Off}
            }
        }
    }
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct AdditionalSignalZs3Symbol(u8);

impl TryInto<track_element::signal::AdditionalSignalZs3Symbol> for &AdditionalSignalZs3Symbol {
    type Error = GenerationError;

    fn try_into(self) -> Result<track_element::signal::AdditionalSignalZs3Symbol, Self::Error> {
        track_element::signal::AdditionalSignalZs3Symbol::try_from(self.0)
            .map_err(|e| GenerationError::InvalidJson(e.to_string()))
    }
}

impl Realize for AdditionalSignalZs3Symbol {
    fn realize(&self) -> TokenStream {
        let symbol: track_element::signal::AdditionalSignalZs3Symbol = self.try_into().unwrap();
        match symbol {
            track_element::signal::AdditionalSignalZs3Symbol::OFF => {
                quote! {track_element::signal::AdditionalSignalZs3Symbol::OFF}
            }
            track_element::signal::AdditionalSignalZs3Symbol::ONE => {
                quote! {track_element::signal::AdditionalSignalZs3Symbol::ONE}
            }
            track_element::signal::AdditionalSignalZs3Symbol::TWO => {
                quote! {track_element::signal::AdditionalSignalZs3Symbol::TWO}
            }
            track_element::signal::AdditionalSignalZs3Symbol::THREE => {
                quote! {track_element::signal::AdditionalSignalZs3Symbol::THREE}
            }
            track_element::signal::AdditionalSignalZs3Symbol::FOUR => {
                quote! {track_element::signal::AdditionalSignalZs3Symbol::FOUR}
            }
            track_element::signal::AdditionalSignalZs3Symbol::FIVE => {
                quote! {track_element::signal::AdditionalSignalZs3Symbol::FIVE}
            }
            track_element::signal::AdditionalSignalZs3Symbol::SIX => {
                quote! {track_element::signal::AdditionalSignalZs3Symbol::SIX}
            }
            track_element::signal::AdditionalSignalZs3Symbol::SEVEN => {
                quote! {track_element::signal::AdditionalSignalZs3Symbol::SEVEN}
            }
            track_element::signal::AdditionalSignalZs3Symbol::EIGHT => {
                quote! {track_element::signal::AdditionalSignalZs3Symbol::EIGHT}
            }
            track_element::signal::AdditionalSignalZs3Symbol::NINE => {
                quote! {track_element::signal::AdditionalSignalZs3Symbol::NINE}
            }
            track_element::signal::AdditionalSignalZs3Symbol::TEN => {
                quote! {track_element::signal::AdditionalSignalZs3Symbol::TEN}
            }
            track_element::signal::AdditionalSignalZs3Symbol::ELEVEN => {
                quote! {track_element::signal::AdditionalSignalZs3Symbol::ELEVEN}
            }
            track_element::signal::AdditionalSignalZs3Symbol::TWELVE => {
                quote! {track_element::signal::AdditionalSignalZs3Symbol::TWELVE}
            }
            track_element::signal::AdditionalSignalZs3Symbol::THIRTEEN => {
                quote! {track_element::signal::AdditionalSignalZs3Symbol::THIRTEEN}
            }
            track_element::signal::AdditionalSignalZs3Symbol::FOURTEEN => {
                quote! {track_element::signal::AdditionalSignalZs3Symbol::FOURTEEN}
            }
            track_element::signal::AdditionalSignalZs3Symbol::FIFTEEN => {
                quote! {track_element::signal::AdditionalSignalZs3Symbol::FIFTEEN}
            }
            track_element::signal::AdditionalSignalZs3Symbol::SIXTEEN => {
                quote! {track_element::signal::AdditionalSignalZs3Symbol::SIXTEEN}
            }
        }
    }
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PointState {
    Left,
    Right,
}

impl Realize for PointState {
    fn realize(&self) -> TokenStream {
        match self {
            PointState::Left => quote! {track_element::point::PointState::Left},
            PointState::Right => quote! {track_element::point::PointState::Right},
        }
    }
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum VacancySectionState {
    Free,
    Occupied,
}

impl Realize for VacancySectionState {
    fn realize(&self) -> TokenStream {
        match self {
            VacancySectionState::Free => {
                quote! {track_element::vacancy_section::VacancySectionState::Free}
            }
            VacancySectionState::Occupied => {
                quote! {track_element::vacancy_section::VacancySectionState::Free}
            }
        }
    }
}
