use std::collections::HashMap;
use quote::quote;
use proc_macro2::TokenStream;

enum TrackElement {
    Point,
    Signal,
}

/// Create new TrackElements and add them to a HashMap
fn realize_element(element: (&str, &TrackElement)) -> TokenStream {
    let (id, kind) = element;
    match kind {
        TrackElement::Point => quote! {
            track_elements.insert(#id, TrackElement::Point(track_element::point::Point::new(track_element::point::PointState::default(), #id)));
        },
        TrackElement::Signal => quote! {
            track_element.insert(#id, TrackElement::Signal(track_element::signal::Signal::new(track_element::signal::SignalState::default(), #id)));
        }
    }
}

fn main() {
    let mut track_elements: HashMap<&str, TrackElement> = HashMap::new();
    let routes: Vec<Vec<(&str, TrackElement)>> = vec![
        vec![
            ("A", TrackElement::Signal),
            ("B", TrackElement::Point),
            ("C", TrackElement::Signal),
        ],
        vec![
            ("B", TrackElement::Point),
            ("C", TrackElement::Signal),
            ("D", TrackElement::Signal),
        ],
        vec![
            ("D", TrackElement::Signal),
            ("E", TrackElement::Point),
            ("F", TrackElement::Signal),
        ],
        vec![
            ("E", TrackElement::Point),
            ("G", TrackElement::Signal),
            ("H", TrackElement::Signal),
        ],
    ];

    for route in routes {
        for (id, elem) in route {
            track_elements.insert(id, elem);
        }
    }
    
    let track_element_tokens: Vec<_> = track_elements.iter().map(|(id, element)| realize_element((*id, element))).collect();

    let tokens = quote!{
        use std::collections::HashMap;

        enum TrackElement {
            Point(track_element::point::Point),
            Signal(track_element::signal::Signal)
        }
        
        fn main(){
            let mut track_elements = HashMap::new();
            #(#track_element_tokens)*
            println!("{:?}", track_elements);
        }
            

    };

    println!("{}", tokens);

}
