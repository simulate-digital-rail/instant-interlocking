use std::{io::Write, path::PathBuf};

use clap::Parser;
use driveway::{DrivewayRepr, TargetState, TrackElement, TrackElementState};
use track_element::{point::PointState, signal::MainSignalState};

mod driveway;
mod generate;

const DEVELOPMENT_ENV: &str = "CODE_GENERATION_DEVELOPMENT_MODE";

#[derive(Debug, Parser)]
#[command(
    name = "IXL Code Generator",
    about = "A tool to generate exceutable interlockings from JSON"
)]
struct Opt {
    /// The JSON source for the generator
    #[arg(value_hint = clap::ValueHint::FilePath, required_unless_present = "example")]
    input: Option<PathBuf>,
    /// Where to write the generated interlocking code
    #[arg(long, short, value_hint = clap::ValueHint::FilePath)]
    output: Option<PathBuf>,
    /// Use the example data provided by this tool (ignores JSON input)
    #[arg(long, short)]
    example: bool,
    /// Development mode: Put the generated interlocking into the cargo examples folder
    #[arg(long)]
    development: bool,
}

fn main() -> anyhow::Result<()> {
    let args = Opt::parse();

    let example_routes: Vec<DrivewayRepr> = vec![
        DrivewayRepr {
            target_state: vec![
                TargetState(
                    "A".to_owned(),
                    TrackElement::Signal(
                        vec![MainSignalState::Hp0, MainSignalState::Ks1],
                        vec![],
                        vec![],
                    ),
                    TrackElementState::Signal((MainSignalState::Ks1).into()),
                ),
                TargetState(
                    "B".to_owned(),
                    TrackElement::Point,
                    TrackElementState::Point(PointState::Left),
                ),
                TargetState(
                    "C".to_owned(),
                    TrackElement::Signal(
                        vec![MainSignalState::Hp0, MainSignalState::Ks1],
                        vec![],
                        vec![],
                    ),
                    TrackElementState::Signal((MainSignalState::Ks1).into()),
                ),
            ],
            start_signal_id: "A".to_owned(),
            end_signal_id: "C".to_owned(),
        },
        DrivewayRepr {
            target_state: vec![
                TargetState(
                    "B".to_owned(),
                    TrackElement::Point,
                    TrackElementState::Point(PointState::Left),
                ),
                TargetState(
                    "C".to_owned(),
                    TrackElement::Signal(
                        vec![MainSignalState::Hp0, MainSignalState::Ks1],
                        vec![],
                        vec![],
                    ),
                    TrackElementState::Signal((MainSignalState::Ks1).into()),
                ),
                TargetState(
                    "D".to_owned(),
                    TrackElement::Signal(
                        vec![MainSignalState::Hp0, MainSignalState::Ks1],
                        vec![],
                        vec![],
                    ),
                    TrackElementState::Signal((MainSignalState::Ks1).into()),
                ),
            ],
            start_signal_id: "C".to_owned(),
            end_signal_id: "D".to_owned(),
        },
        DrivewayRepr {
            target_state: vec![
                TargetState(
                    "D".to_owned(),
                    TrackElement::Signal(
                        vec![MainSignalState::Hp0, MainSignalState::Ks1],
                        vec![],
                        vec![],
                    ),
                    TrackElementState::Signal((MainSignalState::Ks1).into()),
                ),
                TargetState(
                    "E".to_owned(),
                    TrackElement::Point,
                    TrackElementState::Point(PointState::Left),
                ),
                TargetState(
                    "F".to_owned(),
                    TrackElement::Signal(
                        vec![MainSignalState::Hp0, MainSignalState::Ks1],
                        vec![],
                        vec![],
                    ),
                    TrackElementState::Signal((MainSignalState::Ks1).into()),
                ),
            ],
            start_signal_id: "D".to_owned(),
            end_signal_id: "F".to_owned(),
        },
        DrivewayRepr {
            target_state: vec![
                TargetState(
                    "E".to_owned(),
                    TrackElement::Point,
                    TrackElementState::Point(PointState::Left),
                ),
                TargetState(
                    "G".to_owned(),
                    TrackElement::Signal(
                        vec![MainSignalState::Hp0, MainSignalState::Ks1],
                        vec![],
                        vec![],
                    ),
                    TrackElementState::Signal((MainSignalState::Ks1).into()),
                ),
                TargetState(
                    "H".to_owned(),
                    TrackElement::Signal(
                        vec![MainSignalState::Hp0, MainSignalState::Ks1],
                        vec![],
                        vec![],
                    ),
                    TrackElementState::Signal((MainSignalState::Ks1).into()),
                ),
            ],
            start_signal_id: "G".to_owned(),
            end_signal_id: "H".to_owned(),
        },
    ];

    let routes: Vec<_> = if args.example {
        example_routes
    } else {
        let routes_json: serde_json::Value =
            serde_json::from_str(&std::fs::read_to_string(args.input.unwrap())?)?;

        routes_json
            .as_array()
            .unwrap()
            .iter()
            .map(|v| DrivewayRepr::try_from(v).unwrap())
            .collect()
    };

    let generated = generate::generate(&routes)?;
    let generated_tests = generate::generate_tests(&routes)?;

    let is_development = std::env::var(DEVELOPMENT_ENV).is_ok() || args.development;

    let path = if is_development { "examples" } else { "dst" };
    let file_name = if is_development {
        "dev_ixl.rs"
    } else {
        "ixl.rs"
    };
    let file_path = format!("{path}/{file_name}");

    match std::fs::create_dir_all(path) {
        Ok(_) => println!("Created directory {path}"),
        Err(e) => panic!("Could not create directory {path}. Error: {e}"),
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
