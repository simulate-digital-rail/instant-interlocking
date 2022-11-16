use std::io::Write;

use generate::{DrivewayRepr, GenerationError, TrackElement, TrackElementState};
use track_element::{point::PointState, signal::SignalState};

mod generate;

fn main() -> Result<(), GenerationError> {
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
    let generated = generate::generate(routes)?;

    let _ = std::fs::create_dir("dst");
    let mut fp = std::fs::File::create("examples/ixl.rs").unwrap_or_else(|_| {
        std::fs::OpenOptions::new()
            .write(true)
            .open("examples/ixl.rs")
            .unwrap()
    });
    fp.write_all(generated.as_bytes()).unwrap();

    Ok(())
}
