use proc_macro2::TokenStream;
use quote::quote;
use std::collections::HashMap;
use thiserror::Error;
use track_element::{point::PointState, signal::SignalState};

#[derive(Debug, Error)]
pub enum GenerationError {
    #[error("Two track elements with the same ID, but different types exist.")]
    DuplicateTrackElement,
}

#[derive(Clone, Copy)]
pub enum TrackElement {
    Point,
    Signal,
}

#[derive(Clone, Copy)]
pub enum TrackElementState {
    Point(track_element::point::PointState),
    Signal(track_element::signal::SignalState),
}

fn unpack_track_element_signal(id: &str) -> TokenStream {
    quote! {match track_elements.get(#id).unwrap() {TrackElement::Signal(s) => s.clone(), _ => unreachable!() }}
}

fn unpack_track_element_point(id: &str) -> TokenStream {
    quote! {match track_elements.get(#id).unwrap() {TrackElement::Point(p) => p.clone(), _ => unreachable!() }}
}

/// Create new TrackElements and add them to a HashMap
fn realize_element(element: (&str, &TrackElement)) -> TokenStream {
    let (id, kind) = element;
    match kind {
        TrackElement::Point => quote! {
            track_elements.insert(#id, TrackElement::Point(Rc::new(RefCell::new(track_element::point::Point::new(track_element::point::PointState::default(), #id.to_owned())))));
        },
        TrackElement::Signal => quote! {
            track_elements.insert(#id, TrackElement::Signal(Rc::new(RefCell::new(track_element::signal::Signal::new(track_element::signal::SignalState::default(), track_element::signal::SignalType::ToDo, #id.to_owned())))));
        },
    }
}

fn realize_driveway(element_target_states: &DrivewayRepr) -> TokenStream {
    let point_states: Vec<_> = element_target_states
        .target_state
        .iter()
        .filter_map(|(id, _, state)| {
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
        .filter_map(|(id, _, state)| {
            if let TrackElementState::Signal(s) = state {
                let signal = unpack_track_element_signal(id);
                Some(match s {
                    SignalState::Ks1 => quote! {(#signal, track_element::signal::SignalState::Ks1)},
                    SignalState::Ks2 => quote! {(#signal, track_element::signal::SignalState::Ks2)},
                    SignalState::Hp0 => quote! {(#signal, track_element::signal::SignalState::Hp0)},
                })
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
        let target_state = track_element::driveway::TargetState::new(point_states, signal_states);
        let start_signal = #start_signal_tokens;
        let end_signal = #end_signal_tokens;
        driveway_manager.add(Rc::new(RefCell::new(track_element::driveway::Driveway::new(vec![], target_state, start_signal, end_signal))));
    }
}

pub struct DrivewayRepr {
    pub target_state: Vec<(String, TrackElement, TrackElementState)>,
    pub start_signal_id: String,
    pub end_signal_id: String,
}

fn collect_track_elements(
    routes: &Vec<DrivewayRepr>,
) -> Result<HashMap<String, TrackElement>, GenerationError> {
    let mut track_elements: HashMap<String, TrackElement> = HashMap::new();
    for route in routes {
        for (id, elem, _) in &route.target_state {
            if !track_elements.contains_key(id.as_str()) {
                track_elements.insert(id.clone(), *elem);
            } else {
                let existing_track_element = track_elements.get(id.as_str()).unwrap();
                match (existing_track_element, elem) {
                    (TrackElement::Point, TrackElement::Signal) => {
                        return Err(GenerationError::DuplicateTrackElement)
                    }
                    (TrackElement::Signal, TrackElement::Point) => {
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
        let mut track_elements = HashMap::new();
        #(#track_element_tokens)*

        let mut driveway_manager = track_element::driveway::DrivewayManager::new(HashMap::new());
        #(#driveway_tokens)*

        driveway_manager.update_conflicting_driveways();
    }
}

pub fn generate_tests(routes: &Vec<DrivewayRepr>) -> Result<String, GenerationError> {
    let track_elements: HashMap<String, TrackElement> = collect_track_elements(&routes)?;

    let track_element_tokens: Vec<_> = track_elements
        .iter()
        .map(|(id, element)| realize_element((id, element)))
        .collect();

    let driveway_tokens: _ = routes
        .iter()
        .map(|state| realize_driveway(state))
        .collect::<Vec<TokenStream>>();

    let setup_tokens = generate_setup_tokens(track_element_tokens, driveway_tokens);
    
    let tokens = quote! {
        extern crate track_element;

        use std::collections::HashMap;
        use std::cell::RefCell;
        use std::rc::Rc;
        use std::panic;
        use track_element::driveway::DrivewayManager;

        #[derive(Debug)]
        enum TrackElement {
            Point(Rc<RefCell<track_element::point::Point>>),
            Signal(Rc<RefCell<track_element::signal::Signal>>)
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
    let track_elements: HashMap<String, TrackElement> = collect_track_elements(&routes)?;

    let track_element_tokens: Vec<_> = track_elements
        .iter()
        .map(|(id, element)| realize_element((id, element)))
        .collect();
        
        let driveway_tokens: _ = routes
        .iter()
        .map(|state| realize_driveway(state))
        .collect::<Vec<TokenStream>>();

        let setup_tokens = generate_setup_tokens(track_element_tokens, driveway_tokens);

        let tokens = quote! {
            extern crate track_element;
            
            use std::collections::HashMap;
            use std::cell::RefCell;
            use std::rc::Rc;
            
        #[derive(Debug)]
        enum TrackElement {
            Point(Rc<RefCell<track_element::point::Point>>),
            Signal(Rc<RefCell<track_element::signal::Signal>>)
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
