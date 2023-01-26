use proc_macro2::TokenStream;
use quote::quote;
use std::collections::BTreeMap;
use thiserror::Error;
use track_element::{
    additional_signal::AdditionalSignalZs3Symbol, point::PointState, signal::SignalState,
};

use crate::driveway::{DrivewayRepr, TargetState, TrackElement, TrackElementState};

#[derive(Clone, Debug, Error)]
pub enum GenerationError {
    #[error("Two track elements with the same ID, but different types exist.")]
    DuplicateTrackElement,
    #[error("The driveway JSON was not valid.")]
    InvalidJson,
}

fn unpack_track_element_signal(id: &str) -> TokenStream {
    quote! {match track_elements.get(#id).unwrap() {TrackElement::Signal(s) => s.clone(), _ => unreachable!() }}
}

fn unpack_track_element_point(id: &str) -> TokenStream {
    quote! {match track_elements.get(#id).unwrap() {TrackElement::Point(p) => p.clone(), _ => unreachable!() }}
}

fn unpack_track_element_signal_additional_signal_zs3(id: &str) -> TokenStream {
    quote! {match track_elements.get(#id).unwrap() {TrackElement::AdditionalSignalZs3(a) => a.clone(), _ => unreachable!() }}
}

fn quote_additional_signal_zs3_symbol(symbol: &AdditionalSignalZs3Symbol) -> TokenStream {
    match symbol {
        track_element::additional_signal::AdditionalSignalZs3Symbol::OFF => {
            quote! {track_element::additional_signal::AdditionalSignalZs3Symbol::OFF}
        }
        track_element::additional_signal::AdditionalSignalZs3Symbol::ONE => {
            quote! {track_element::additional_signal::AdditionalSignalZs3Symbol::ONE}
        }
        track_element::additional_signal::AdditionalSignalZs3Symbol::TWO => {
            quote! {track_element::additional_signal::AdditionalSignalZs3Symbol::TWO}
        }
        track_element::additional_signal::AdditionalSignalZs3Symbol::THREE => {
            quote! {track_element::additional_signal::AdditionalSignalZs3Symbol::THREE}
        }
        track_element::additional_signal::AdditionalSignalZs3Symbol::FOUR => {
            quote! {track_element::additional_signal::AdditionalSignalZs3Symbol::FOUR}
        }
        track_element::additional_signal::AdditionalSignalZs3Symbol::FIVE => {
            quote! {track_element::additional_signal::AdditionalSignalZs3Symbol::FIVE}
        }
        track_element::additional_signal::AdditionalSignalZs3Symbol::SIX => {
            quote! {track_element::additional_signal::AdditionalSignalZs3Symbol::SIX}
        }
        track_element::additional_signal::AdditionalSignalZs3Symbol::SEVEN => {
            quote! {track_element::additional_signal::AdditionalSignalZs3Symbol::SEVEN}
        }
        track_element::additional_signal::AdditionalSignalZs3Symbol::EIGHT => {
            quote! {track_element::additional_signal::AdditionalSignalZs3Symbol::EIGHT}
        }
        track_element::additional_signal::AdditionalSignalZs3Symbol::NINE => {
            quote! {track_element::additional_signal::AdditionalSignalZs3Symbol::NINE}
        }
        track_element::additional_signal::AdditionalSignalZs3Symbol::TEN => {
            quote! {track_element::additional_signal::AdditionalSignalZs3Symbol::TEN}
        }
        track_element::additional_signal::AdditionalSignalZs3Symbol::ELEVEN => {
            quote! {track_element::additional_signal::AdditionalSignalZs3Symbol::ELEVEN}
        }
        track_element::additional_signal::AdditionalSignalZs3Symbol::TWELVE => {
            quote! {track_element::additional_signal::AdditionalSignalZs3Symbol::TWELVE}
        }
        track_element::additional_signal::AdditionalSignalZs3Symbol::THIRTEEN => {
            quote! {track_element::additional_signal::AdditionalSignalZs3Symbol::THIRTEEN}
        }
        track_element::additional_signal::AdditionalSignalZs3Symbol::FOURTEEN => {
            quote! {track_element::additional_signal::AdditionalSignalZs3Symbol::FOURTEEN}
        }
        track_element::additional_signal::AdditionalSignalZs3Symbol::FIFTEEN => {
            quote! {track_element::additional_signal::AdditionalSignalZs3Symbol::FIFTEEN}
        }
        track_element::additional_signal::AdditionalSignalZs3Symbol::SIXTEEN => {
            quote! {track_element::additional_signal::AdditionalSignalZs3Symbol::SIXTEEN}
        }
    }
}

/// Create new TrackElements and add them to a BTreeMap
fn realize_element(element: (&str, &TrackElement)) -> TokenStream {
    let (id, kind) = element;
    match kind {
        TrackElement::Point => quote! {
            track_elements.insert(#id, TrackElement::Point(Rc::new(RefCell::new(track_element::point::Point::new(track_element::point::PointState::default(), #id.to_owned())))));
        },
        TrackElement::Signal => quote! {
            track_elements.insert(#id, TrackElement::Signal(Rc::new(RefCell::new(track_element::signal::Signal::new(track_element::signal::SignalState::default(), track_element::signal::SignalType::ToDo, #id.to_owned())))));
        },
        TrackElement::AdditionalSignalZs3(is_v, symbols) => {
            let symbol_tokens: Vec<_> = symbols
                .iter()
                .map(quote_additional_signal_zs3_symbol)
                .collect();

            quote! {
                track_elements.insert(#id, TrackElement::AdditionalSignalZs3(Rc::new(RefCell::new(track_element::additional_signal::AdditionalSignalZs3::new(#is_v, vec![#(#symbol_tokens),*], track_element::additional_signal::AdditionalSignalZs3Symbol::default(), #id.to_owned())))));
            }
        }
    }
}

fn realize_driveway(element_target_states: &DrivewayRepr) -> TokenStream {
    let point_states: Vec<_> = element_target_states
        .target_state
        .iter()
        .filter_map(|TargetState(id, _, state)| {
            if let TrackElementState::Point(p) = state {
                let point = unpack_track_element_point(id);
                Some(match p {
                    PointState::Left => {
                        quote! {(#point, track_element::point::PointState::Left)}
                    }
                    PointState::Right => {
                        quote! {(#point, track_element::point::PointState::Right)}
                    }
                })
            } else {
                None
            }
        })
        .collect();
    let signal_states: Vec<_> = element_target_states
        .target_state
        .iter()
        .filter_map(|TargetState(id, _, state)| {
            if let TrackElementState::Signal(s) = state {
                let signal = unpack_track_element_signal(id);
                Some(match s {
                    SignalState::Ks1 => quote! {(#signal, track_element::signal::SignalState::Ks1)},
                    SignalState::Ks2 => quote! {(#signal, track_element::signal::SignalState::Ks2)},
                    SignalState::Hp0 => quote! {(#signal, track_element::signal::SignalState::Hp0)},
                    SignalState::Hp0PlusSh1 => quote! {(#signal, track_element::signal::SignalState::Hp0PlusSh1)},
                    SignalState::Hp0WithDrivingIndicator => quote! {(#signal, track_element::signal::SignalState::Hp0WithDrivingIndicator)},
                    SignalState::Ks1Flashing => quote! {(#signal, track_element::signal::SignalState::Ks1Flashing)},
                    SignalState::Ks1FlashingWithAdditionalLight => quote! {(#signal, track_element::signal::SignalState::Ks1FlashingWithAdditionalLight)},
                    SignalState::Ks2WithAdditionalLight => quote! {(#signal, track_element::signal::SignalState::Ks2WithAdditionalLight)},
                    SignalState::Sh1 => quote! {(#signal, track_element::signal::SignalState::Sh1)},
                    SignalState::IdLight => quote! {(#signal, track_element::signal::SignalState::IdLight)},
                    SignalState::Hp0Hv => quote! {(#signal, track_element::signal::SignalState::Hp0Hv)},
                    SignalState::Hp1 => quote! {(#signal, track_element::signal::SignalState::Hp1)},
                    SignalState::Hp2 => quote! {(#signal, track_element::signal::SignalState::Hp2)},
                    SignalState::Vr0 => quote! {(#signal, track_element::signal::SignalState::Vr0)},
                    SignalState::Vr1 => quote! {(#signal, track_element::signal::SignalState::Vr1)},
                    SignalState::Vr2 => quote! {(#signal, track_element::signal::SignalState::Vr2)},
                    SignalState::Off => quote! {(#signal, track_element::signal::SignalState::Off)},
                })
            } else {
                None
            }
        })
        .collect();
    let additional_signal_zs3_states: Vec<_> = element_target_states
        .target_state
        .iter()
        .filter_map(|TargetState(id, _, state)| {
            if let TrackElementState::AdditionalSignal(a) = state {
                let additional_signal = unpack_track_element_signal_additional_signal_zs3(id);
                let symbol = quote_additional_signal_zs3_symbol(a);
                Some(quote! {(#additional_signal, #symbol)})
            } else {
                None
            }
        })
        .collect();
    let start_signal_id = &element_target_states.start_signal_id;
    let start_signal_tokens = unpack_track_element_signal(start_signal_id);
    let end_signal_id = &element_target_states.end_signal_id;
    let end_signal_tokens = unpack_track_element_signal(end_signal_id);
    quote! {
        let point_states = vec![#(#point_states),*];
        let signal_states = vec![#(#signal_states),*];
        let additional_signal_zs3_states = vec![#(#additional_signal_zs3_states),*];
        let target_state = track_element::driveway::TargetState::new(point_states, signal_states, additional_signal_zs3_states);
        let start_signal = #start_signal_tokens;
        let end_signal = #end_signal_tokens;
        driveway_manager.add(Rc::new(RefCell::new(track_element::driveway::Driveway::new(vec![], target_state, start_signal, end_signal))));
    }
}

fn collect_track_elements(
    routes: &Vec<DrivewayRepr>,
) -> Result<BTreeMap<String, TrackElement>, GenerationError> {
    let mut track_elements: BTreeMap<String, TrackElement> = BTreeMap::new();
    for route in routes {
        for TargetState(id, elem, _) in &route.target_state {
            if !track_elements.contains_key(id.as_str()) {
                track_elements.insert(id.clone(), elem.clone());
            } else {
                let existing_track_element = track_elements.get(id.as_str()).unwrap();
                match (existing_track_element, elem) {
                    (TrackElement::Point, TrackElement::Signal) => {
                        return Err(GenerationError::DuplicateTrackElement)
                    }
                    (TrackElement::Signal, TrackElement::Point) => {
                        return Err(GenerationError::DuplicateTrackElement)
                    }

                    (TrackElement::Point, TrackElement::AdditionalSignalZs3(_, _)) => {
                        return Err(GenerationError::DuplicateTrackElement)
                    }

                    (TrackElement::Signal, TrackElement::AdditionalSignalZs3(_, _)) => {
                        return Err(GenerationError::DuplicateTrackElement)
                    }
                    (TrackElement::AdditionalSignalZs3(_, _), TrackElement::Point) => {
                        return Err(GenerationError::DuplicateTrackElement)
                    }
                    (TrackElement::AdditionalSignalZs3(_, _), TrackElement::Signal) => {
                        return Err(GenerationError::DuplicateTrackElement)
                    }
                    _ => {}
                }
            }
        }
    }
    Ok(track_elements)
}

fn generate_setup_tokens(
    track_element_tokens: Vec<TokenStream>,
    driveway_tokens: Vec<TokenStream>,
) -> TokenStream {
    quote! {
        let mut track_elements = BTreeMap::new();
        #(#track_element_tokens)*

        let mut driveway_manager = track_element::driveway::DrivewayManager::new(BTreeMap::new());
        #(#driveway_tokens)*

        driveway_manager.update_conflicting_driveways();
    }
}

pub fn generate_tests(routes: &Vec<DrivewayRepr>) -> Result<String, GenerationError> {
    let track_elements: BTreeMap<String, TrackElement> = collect_track_elements(routes)?;

    let track_element_tokens: Vec<_> = track_elements
        .iter()
        .map(|(id, element)| realize_element((id, element)))
        .collect();

    let driveway_tokens: _ = routes
        .iter()
        .map(realize_driveway)
        .collect::<Vec<TokenStream>>();

    let setup_tokens = generate_setup_tokens(track_element_tokens, driveway_tokens);

    let tokens = quote! {
        extern crate track_element;

        use std::collections::BTreeMap;
        use std::cell::RefCell;
        use std::rc::Rc;
        use std::panic;
        use track_element::driveway::DrivewayManager;

        #[derive(Debug)]
        enum TrackElement {
            Point(Rc<RefCell<track_element::point::Point>>),
            Signal(Rc<RefCell<track_element::signal::Signal>>),
            AdditionalSignalZs3(Rc<RefCell<track_element::additional_signal::AdditionalSignalZs3>>),
        }

        #[test]
        fn test_known_driveway() {
            #setup_tokens
            assert!(driveway_manager.set_driveway("A","C").is_ok());
        }

        #[test]
        fn test_unknown_driveway() {
            #setup_tokens
            assert!(driveway_manager.set_driveway("A","X").is_err());
        }

    };

    Ok(tokens.to_string())
}

pub fn generate(routes: &Vec<DrivewayRepr>) -> Result<String, GenerationError> {
    let track_elements: BTreeMap<String, TrackElement> = collect_track_elements(routes)?;

    let track_element_tokens: Vec<_> = track_elements
        .iter()
        .map(|(id, element)| realize_element((id, element)))
        .collect();

    let driveway_tokens: _ = routes
        .iter()
        .map(realize_driveway)
        .collect::<Vec<TokenStream>>();

    let setup_tokens = generate_setup_tokens(track_element_tokens, driveway_tokens);

    let tokens = quote! {
            extern crate track_element;

            use std::collections::BTreeMap;
            use std::cell::RefCell;
            use std::rc::Rc;

        #[derive(Debug)]
        enum TrackElement {
            Point(Rc<RefCell<track_element::point::Point>>),
            Signal(Rc<RefCell<track_element::signal::Signal>>),
            AdditionalSignalZs3(Rc<RefCell<track_element::additional_signal::AdditionalSignalZs3>>)
        }

        fn main(){
            #setup_tokens

            println!("TrackElements: {:?}", track_elements);
            println!("Driveways: {:?}", driveway_manager.get_driveway_ids().collect::<Vec<_>>());

            let control_station = track_element::control_station::ControlStation::new(driveway_manager);
            control_station.start();
        }
    };

    Ok(tokens.to_string())
}
