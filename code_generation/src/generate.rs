use proc_macro2::TokenStream;
use quote::quote;
use std::{cmp::Ordering, collections::BTreeMap};
use thiserror::Error;

use crate::{
    driveway::{DrivewayRepr, Realize, TrackElement},
    ControlStation,
};

#[derive(Clone, Debug, Error)]
pub enum GenerationError {
    #[error("Two track elements with the same ID, but different types exist.")]
    DuplicateTrackElement,
    #[error("The driveway JSON was not valid.")]
    InvalidJson(String),
}

/// Create new TrackElements and add them to a BTreeMap
fn realize_element(element: &TrackElement) -> TokenStream {
    let realized = element.realize();
    match element {
        TrackElement::Point { uuid, .. } => quote! {
            track_elements.insert(#uuid, TrackElement::Point(#realized));
        },
        TrackElement::Signal { uuid, .. } => {
            quote! {
                track_elements.insert(#uuid, TrackElement::Signal(#realized));
            }
        }
        TrackElement::VacancySection { uuid, .. } => {
            quote! {
                track_elements.insert(#uuid, TrackElement::VacancySection(#realized));
            }
        }
    }
}

fn realize_driveway(element_target_states: &DrivewayRepr) -> TokenStream {
    let point_states: Vec<_> = element_target_states
        .states
        .iter()
        .filter_map(|e| {
            if let TrackElement::Point { uuid, state, .. } = e {
                let point = quote! {
                    match track_elements.get(#uuid).unwrap() {
                        TrackElement::Point(point) => point.clone(),
                        _ => unreachable!()
                    }
                };
                let state = state.realize();
                Some(quote! {(#point, #state)})
            } else {
                None
            }
        })
        .collect();

    let signal_states: Vec<_> = element_target_states
        .states
        .iter()
        .filter_map(|e| {
            if let TrackElement::Signal { uuid, state, .. } = e {
                let signal = quote! {
                    match track_elements.get(#uuid).unwrap() {
                        TrackElement::Signal(signal) => signal.clone(),
                        _ => unreachable!()
                    }
                };
                let state = state.realize();
                Some(quote! {(#signal, #state)})
            } else {
                None
            }
        })
        .collect();

    let vacancy_section_states: Vec<_> = element_target_states
        .states
        .iter()
        .filter_map(|e| {
            if let TrackElement::VacancySection { uuid, state, .. } = e {
                let vacancy_section = quote! { match track_elements.get(#uuid).unwrap() {
                    TrackElement::VacancySection(vacancy_section) => vacancy_section.clone(),
                    _ => unreachable!()
                } };
                let state = state.realize();
                Some(quote! {(#vacancy_section, #state)})
            } else {
                None
            }
        })
        .collect();

    let start_signal_id = &element_target_states.start_signal;
    let end_signal_id = &element_target_states.end_signal;

    quote! {
        let point_states = vec![#(#point_states),*];
        let signal_states = vec![#(#signal_states),*];
        let vacancy_section_states = vec![#(#vacancy_section_states),*];
        let target_state = track_element::driveway::DrivewayState::new(point_states, signal_states, vacancy_section_states);
        driveway_manager.add(Arc::new(RwLock::new(track_element::driveway::Driveway::new(vec![], target_state, #start_signal_id.to_string(), #end_signal_id.to_string()))));
    }
}

fn collect_track_elements(
    routes: &Vec<DrivewayRepr>,
) -> Result<BTreeMap<String, TrackElement>, GenerationError> {
    let mut track_elements: BTreeMap<String, TrackElement> = BTreeMap::new();
    for route in routes {
        for el in &route.states {
            let id = el.id();
            if !track_elements.contains_key(id) {
                track_elements.insert(id.to_string(), el.clone());
            } else {
                let existing_track_element = track_elements.get(id).unwrap();
                match (existing_track_element, el) {
                    (TrackElement::Point { .. }, TrackElement::Point { .. }) => {}
                    (TrackElement::Signal { .. }, TrackElement::Signal { .. }) => {}
                    (TrackElement::VacancySection { .. }, TrackElement::VacancySection { .. }) => {}
                    _ => return Err(GenerationError::DuplicateTrackElement),
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
    let mut track_elements: Vec<TrackElement> =
        collect_track_elements(routes)?.into_values().collect();

    track_elements.sort_by(|a, b| match (a, b) {
        (_, TrackElement::VacancySection { .. }) => Ordering::Less,
        _ => Ordering::Equal,
    });

    let track_element_tokens: Vec<_> = track_elements
        .iter()
        .map(|element| realize_element(element))
        .collect();

    let driveway_tokens: _ = routes
        .iter()
        .map(realize_driveway)
        .collect::<Vec<TokenStream>>();

    let setup_tokens = generate_setup_tokens(track_element_tokens, driveway_tokens);

    let tokens = quote! {
        extern crate track_element;

        use std::collections::BTreeMap;
        use std::sync::{Arc, RwLock};
        use std::panic;
        use track_element::driveway::DrivewayManager;

        #[derive(Debug)]
        enum TrackElement {
            Point(Arc<RwLock<track_element::point::Point>>),
            Signal(Arc<RwLock<track_element::signal::Signal>>),
            VacancySection(Arc<RwLock<track_element::vacancy_section::VacancySection>>)
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

fn generate_control_station(control_station: &ControlStation) -> TokenStream {
    match control_station {
        ControlStation::Cli => quote! {
            let control_station = track_element::control_station::ControlStation::new(driveway_manager);
            control_station.start();
        },
        ControlStation::Grpc {
            addr,
            topology,
            placement,
        } => {
            let topology = std::fs::read_to_string(topology)
                .expect("Topology should point to a valid JSON file");
            let placement = std::fs::read_to_string(placement)
                .expect("Placement should point to a valid JSON file");

            quote! {
                let mut control_station = grpc_control_station::ControlStation::new(driveway_manager, #topology, #placement);

                let addr = #addr.parse().unwrap();
                control_station.listen(addr).await.unwrap();
            }
        }
    }
}

pub fn generate(routes: &Vec<DrivewayRepr>, cs: ControlStation) -> Result<String, GenerationError> {
    let mut track_elements: Vec<TrackElement> =
        collect_track_elements(routes)?.into_values().collect();

    track_elements.sort_by(|a, b| match (a, b) {
        (_, TrackElement::VacancySection { .. }) => Ordering::Less,
        _ => Ordering::Equal,
    });

    let track_element_tokens: Vec<_> = track_elements
        .iter()
        .map(|element| realize_element(element))
        .collect();

    let driveway_tokens: _ = routes
        .iter()
        .map(realize_driveway)
        .collect::<Vec<TokenStream>>();

    let setup_tokens = generate_setup_tokens(track_element_tokens, driveway_tokens);

    let control_station = generate_control_station(&cs);

    let (main_qualifier, main_attr) = match cs {
        ControlStation::Cli => (quote! {}, quote! {}),
        ControlStation::Grpc { .. } => (quote! {async}, quote! {#[tokio::main]}),
    };

    let tokens = quote! {
            extern crate track_element;

            #[cfg(test)]
            mod test;

            use std::collections::BTreeMap;
            use std::sync::{Arc, RwLock};

        #[derive(Debug)]
        enum TrackElement {
            Point(Arc<RwLock<track_element::point::Point>>),
            Signal(Arc<RwLock<track_element::signal::Signal>>),
            VacancySection(Arc<RwLock<track_element::vacancy_section::VacancySection>>)
        }

        #main_attr
        #main_qualifier fn main() {
            #setup_tokens

            println!("TrackElements: {track_elements:?}");
            println!("Driveways: {:?}", driveway_manager.get_driveway_ids().collect::<Vec<_>>());

            #control_station
        }
    };

    Ok(tokens.to_string())
}
