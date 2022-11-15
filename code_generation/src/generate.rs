use proc_macro2::TokenStream;
use quote::quote;
use std::collections::HashMap;
use std::io::Write;
use track_element::{point::PointState, signal::SignalState};

#[derive(Clone, Copy)]
enum TrackElement {
    Point,
    Signal,
}

#[derive(Clone, Copy)]
enum TrackElementState {
    Point(track_element::point::PointState),
    Signal(track_element::signal::SignalState),
}

fn unpack_track_element_signal(id: &str) -> TokenStream {
    quote!{match track_elements.get(#id).unwrap() {TrackElement::Signal(s) => s.clone(), _ => unreachable!() }}
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
    let point_states: Vec<_> = element_target_states.target_state
        .iter()
        .filter_map(|(id, _, state)| {
            if let TrackElementState::Point(p) = state {
                Some(match p {
                    PointState::Left => {
                        quote! {(match track_elements.get(#id).unwrap() {TrackElement::Point(p) => p.clone(), _ => unreachable!() }, track_element::point::PointState::Left)}
                    },
                    PointState::Right => {
                        quote! {(match track_elements.get(#id).unwrap() {TrackElement::Point(p) => p.clone(), _ => unreachable!() }, track_element::point::PointState::Right)}
                    },
                })
            } else {
                None
            }
        })
        .collect();
    let signal_states: Vec<_> = element_target_states.target_state
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

struct DrivewayRepr {
    target_state: Vec<(String, TrackElement, TrackElementState)>,
    start_signal_id: String,
    end_signal_id: String,
}

pub fn generator_example() {
    let mut track_elements: HashMap<&str, TrackElement> = HashMap::new();
    let routes: Vec<DrivewayRepr> = vec![
        DrivewayRepr {
            target_state: vec![
                (
                    "A".to_owned(),
                    TrackElement::Signal,
                    TrackElementState::Signal(SignalState::Ks1),
                ),
                (
                    "B".to_owned(),
                    TrackElement::Point,
                    TrackElementState::Point(PointState::Left),
                ),
                (
                    "C".to_owned(),
                    TrackElement::Signal,
                    TrackElementState::Signal(SignalState::Ks1),
                ),
            ],
            start_signal_id: "A".to_owned(),
            end_signal_id: "C".to_owned(),
        },
        DrivewayRepr {
            target_state: vec![
                (
                    "B".to_owned(),
                    TrackElement::Point,
                    TrackElementState::Point(PointState::Left),
                ),
                (
                    "C".to_owned(),
                    TrackElement::Signal,
                    TrackElementState::Signal(SignalState::Ks1),
                ),
                (
                    "D".to_owned(),
                    TrackElement::Signal,
                    TrackElementState::Signal(SignalState::Ks1),
                ),
            ],
            start_signal_id: "C".to_owned(),
            end_signal_id: "D".to_owned(),
        },
        DrivewayRepr {
            target_state: vec![
                (
                    "D".to_owned(),
                    TrackElement::Signal,
                    TrackElementState::Signal(SignalState::Ks1),
                ),
                (
                    "E".to_owned(),
                    TrackElement::Point,
                    TrackElementState::Point(PointState::Left),
                ),
                (
                    "F".to_owned(),
                    TrackElement::Signal,
                    TrackElementState::Signal(SignalState::Ks1),
                ),
            ],
            start_signal_id: "D".to_owned(),
            end_signal_id: "F".to_owned(),
        },
        DrivewayRepr {
            target_state: vec![
                (
                    "E".to_owned(),
                    TrackElement::Point,
                    TrackElementState::Point(PointState::Left),
                ),
                (
                    "G".to_owned(),
                    TrackElement::Signal,
                    TrackElementState::Signal(SignalState::Ks1),
                ),
                (
                    "H".to_owned(),
                    TrackElement::Signal,
                    TrackElementState::Signal(SignalState::Ks1),
                ),
            ],
            start_signal_id: "G".to_owned(),
            end_signal_id: "H".to_owned(),
        },
    ];

    for route in &routes {
        for (id, elem, _) in &route.target_state {
            track_elements.insert(&id, *elem);
        }
    }

    let track_element_tokens: Vec<_> = track_elements
        .iter()
        .map(|(id, element)| realize_element((*id, element)))
        .collect();

    let driveway_tokens: _ = routes.iter().map(|route| realize_driveway(route));

    let tokens = quote! {
        use std::collections::HashMap;
        use std::cell::RefCell;
        use std::rc::Rc;

        #[derive(Debug)]
        enum TrackElement {
            Point(Rc<RefCell<track_element::point::Point>>),
            Signal(Rc<RefCell<track_element::signal::Signal>>)
        }

        fn main(){
            let mut track_elements = HashMap::new();
            #(#track_element_tokens)*

            let mut driveway_manager = track_element::driveway::DrivewayManager::new(HashMap::new());
            #(#driveway_tokens)*

            driveway_manager.update_conflicting_driveways();

            println!("{:?}", track_elements);
            println!("{:?}", driveway_manager.get_driveway_ids().collect::<Vec<_>>());

        }
    };

    //println!("{}", tokens);
    let _ = std::fs::create_dir("dst");
    let mut fp = std::fs::File::create("examples/ixl.rs").unwrap_or_else(|_| {
        std::fs::OpenOptions::new()
            .write(true)
            .open("examples/ixl.rs")
            .unwrap()
    });
    fp.write_all(tokens.to_string().as_bytes()).unwrap();
}
