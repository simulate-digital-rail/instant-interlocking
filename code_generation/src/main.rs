use std::{io::Write};

use generate::{DrivewayRepr, GenerationError, TrackElement, TrackElementState};
use track_element::{point::PointState, signal::SignalState};

mod generate;

const DEVELOPMENT_ENV: &str = "CODE_GENERATION_DEVELOPMENT_MODE";

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
    let generated = generate::generate(&routes)?;
    let generated_tests = generate::generate_tests(&routes)?;

    let mut is_development = false;
    match std::env::var(DEVELOPMENT_ENV) {
        Ok(_) => {
            is_development = true;
            println!("Development mode")},
        Err(_) => ()
    }

    let path = if is_development {"examples"} else {"dst"};
    let file_name = if is_development {"dev_ixl.rs"} else {"ixl.rs"};
    let file_path = format!("{}/{}", path, file_name);

    match std::fs::create_dir_all(path) {
        Ok(_) => println!("Created directory {}", path),
        Err(e) => panic!("Could not create directory {}. {}",path, e)
    }

    // Interlocking
    let mut fp = std::fs::File::create(file_path.clone()).unwrap_or_else(|_| {
        std::fs::OpenOptions::new()
            .write(true)
            .open(file_path)
            .unwrap()
    });
    fp.write_all(generated.as_bytes()).unwrap();

    // Tests
    let mut fp = std::fs::File::create("src/dev_test.rs").unwrap_or_else(|_| {
        std::fs::OpenOptions::new()
            .write(true)
            .open("src/dev_test.rs")
            .unwrap()
    });
    fp.write_all(generated_tests.as_bytes()).unwrap();

    Ok(())
}
