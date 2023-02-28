use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens};
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

pub fn uuid_to_var_name(uuid: &str) -> TokenStream {
    format_ident!("_{}", uuid.replace('-', "_")).to_token_stream()
}

/// Create new TrackElements and add them to a BTreeMap
fn realize_element(element: &TrackElement) -> TokenStream {
    let var_name = uuid_to_var_name(element.id());
    let realized = element.realize();
    quote! {let #var_name = #realized;}
}

fn realize_driveway(element_target_states: &DrivewayRepr) -> TokenStream {
    let point_states: Vec<_> = element_target_states
        .states
        .iter()
        .filter_map(|e| {
            if let TrackElement::Point { uuid, state, .. } = e {
                let point = uuid_to_var_name(uuid);
                let state = state.realize();
                Some(quote! {(#point.clone(), #state)})
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
                let signal = uuid_to_var_name(uuid);
                let state = state.realize();
                Some(quote! {(#signal.clone(), #state)})
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
                let vacancy_section = uuid_to_var_name(uuid);
                let state = state.realize();
                Some(quote! {(#vacancy_section.clone(), #state)})
            } else {
                None
            }
        })
        .collect();

    let start_signal = uuid_to_var_name(element_target_states.start_signal.id());
    let end_signal = uuid_to_var_name(element_target_states.end_signal.id());

    quote! {
        let point_states = vec![#(#point_states),*];
        let signal_states = vec![#(#signal_states),*];
        let vacancy_section_states = vec![#(#vacancy_section_states),*];
        let target_state = track_element::driveway::DrivewayState::new(point_states, signal_states, vacancy_section_states);
        driveway_manager.add(Arc::new(RwLock::new(track_element::driveway::Driveway::new(vec![], target_state, #start_signal.clone(), #end_signal.clone()))));
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

            if let TrackElement::VacancySection {
                previous_signals, ..
            } = el
            {
                for signal in previous_signals {
                    if !track_elements.contains_key(signal.id()) {
                        track_elements.insert(signal.id().to_string(), signal.clone());
                    }
                }
            }
        }
        if !track_elements.contains_key(route.start_signal.id()) {
            track_elements.insert(
                route.start_signal.id().to_string(),
                route.start_signal.clone(),
            );
        }
        if !track_elements.contains_key(route.end_signal.id()) {
            track_elements.insert(route.end_signal.id().to_string(), route.end_signal.clone());
        }
    }
    Ok(track_elements)
}

fn generate_setup_tokens(
    track_element_tokens: Vec<TokenStream>,
    driveway_tokens: Vec<TokenStream>,
) -> TokenStream {
    quote! {
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

    let track_element_tokens: Vec<_> = track_elements.iter().map(realize_element).collect();

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

    let track_element_tokens: Vec<_> = track_elements.iter().map(realize_element).collect();

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



        #main_attr
        #main_qualifier fn main() {
            #setup_tokens

            println!("Driveways: {:?}", driveway_manager.get_driveway_ids());

            #control_station
        }
    };

    Ok(tokens.to_string())
}
