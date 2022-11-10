use proc_macro2::TokenStream;
use quote::quote;
use std::collections::HashMap;
use track_element::{point::PointState, signal::SignalState};
use std::io::Write;

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

/// Create new TrackElements and add them to a HashMap
fn realize_element(element: (&str, &TrackElement)) -> TokenStream {
    let (id, kind) = element;
    match kind {
        TrackElement::Point => quote! {
            track_elements.insert(#id, TrackElement::Point(track_element::point::Point::new(track_element::point::PointState::default(), #id)));
        },
        TrackElement::Signal => quote! {
            track_element.insert(#id, TrackElement::Signal(track_element::signal::Signal::new(track_element::signal::SignalState::default(), track_element::signal::SignalType::ToDo, #id)));
        },
    }
}

fn realize_driveway(element_target_states: &[(&str, TrackElement, TrackElementState)]) -> TokenStream {
    let point_states: Vec<_> = element_target_states.iter().filter_map(|(_,_,state)| if let TrackElementState::Point(p) = state {Some(p)} else {None}).collect();
    let signal_states: Vec<_> = element_target_states.iter().filter_map(|(_,_,state)| if let TrackElementState::Signal(s) = state {Some(s)} else {None}).collect();
    quote! {
            let target_state = track_element::driveway::TargetState::new(#point_states, #signal_states);
            driveway_manager.add(Rc::new(RefCell::new(track_element::driveway::Driveway::new(vec![], target_state))));
    }
}

pub fn generator_example() {
    let mut track_elements: HashMap<&str, TrackElement> = HashMap::new();
    let routes: Vec<Vec<(&str, TrackElement, TrackElementState)>> = vec![
        vec![
            (
                "A",
                TrackElement::Signal,
                TrackElementState::Signal(SignalState::Ks1),
            ),
            (
                "B",
                TrackElement::Point,
                TrackElementState::Point(PointState::Left),
            ),
            (
                "C",
                TrackElement::Signal,
                TrackElementState::Point(PointState::Left),
            ),
        ],
        vec![
            (
                "B",
                TrackElement::Point,
                TrackElementState::Point(PointState::Left),
            ),
            (
                "C",
                TrackElement::Signal,
                TrackElementState::Signal(SignalState::Ks1),
            ),
            (
                "D",
                TrackElement::Signal,
                TrackElementState::Signal(SignalState::Ks1),
            ),
        ],
        vec![
            (
                "D",
                TrackElement::Signal,
                TrackElementState::Signal(SignalState::Ks1),
            ),
            (
                "E",
                TrackElement::Point,
                TrackElementState::Point(PointState::Left),
            ),
            (
                "F",
                TrackElement::Signal,
                TrackElementState::Signal(SignalState::Ks1),
            ),
        ],
        vec![
            (
                "E",
                TrackElement::Point,
                TrackElementState::Point(PointState::Left),
            ),
            (
                "G",
                TrackElement::Signal,
                TrackElementState::Signal(SignalState::Ks1),
            ),
            (
                "H",
                TrackElement::Signal,
                TrackElementState::Signal(SignalState::Ks1),
            ),
        ],
    ];

    for route in &routes {
        for (id, elem, _) in route {
            track_elements.insert(id, *elem);
        }
    }

    let track_element_tokens: Vec<_> = track_elements
        .iter()
        .map(|(id, element)| realize_element((*id, element)))
        .collect();

    let driveway_tokens: _ = routes.iter().map(|route| realize_driveway(route.as_slice()));

    let tokens = quote! {
        use std::collections::HashMap;

        enum TrackElement {
            Point(track_element::point::Point),
            Signal(track_element::signal::Signal)
        }

        fn main(){
            let mut track_elements = HashMap::new();
            #(#track_element_tokens)*
            
            let driveway_manager = DrivewayManager::new(HashMap::new())
            #(#driveway_tokens)*
            
            println!("{:?}", track_elements);
        }
    };

    println!("{}", tokens);
    std::fs::create_dir("../dst").unwrap();
    let mut fp = std::fs::File::create("../dst/ixl.rs").unwrap();
    fp.write_all(tokens.to_string().as_bytes()).unwrap();
}
