use generate::{DrivewayRepr, TrackElement, TrackElementState};
use track_element::{point::PointState, signal::SignalState};

mod generate;

fn main() {
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
    generate::generate(routes);
}
