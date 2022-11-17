use std::{fs, io::Write, path::PathBuf};

use driveway::{DrivewayRepr, TargetState, TrackElement, TrackElementState};
use structopt::StructOpt;
use track_element::{point::PointState, signal::SignalState};

mod driveway;
mod generate;

const DEVELOPMENT_ENV: &str = "CODE_GENERATION_DEVELOPMENT_MODE";

#[derive(Debug, StructOpt)]
#[structopt(
    name = "IXL Code Generator",
    about = "A tool to generate exceutable interlockings from JSON"
)]
struct Opt {
    /// The JSON source for the generator
    #[structopt(parse(from_os_str), required_unless = "example")]
    input: Option<PathBuf>,
    /// Where to write the generated interlocking code
    #[structopt(long, short, parse(from_os_str))]
    output: Option<PathBuf>,
    /// Use the example data provided by this tool
    #[structopt(long, short)]
    example: bool,
    /// Development mode: Put the generated interlocking into the cargo examples folder
    #[structopt(long, short = "dev")]
    development: bool,
}

fn main() -> anyhow::Result<()> {
    let args = Opt::from_args();

    let example_routes: Vec<DrivewayRepr> = vec![
        DrivewayRepr {
            target_state: vec![
                TargetState(
                    "A".to_owned(),
                    TrackElement::Signal,
                    TrackElementState::Signal(SignalState::Ks1),
                ),
                TargetState(
                    "B".to_owned(),
                    TrackElement::Point,
                    TrackElementState::Point(PointState::Left),
                ),
                TargetState(
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
                TargetState(
                    "B".to_owned(),
                    TrackElement::Point,
                    TrackElementState::Point(PointState::Left),
                ),
                TargetState(
                    "C".to_owned(),
                    TrackElement::Signal,
                    TrackElementState::Signal(SignalState::Ks1),
                ),
                TargetState(
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
                TargetState(
                    "D".to_owned(),
                    TrackElement::Signal,
                    TrackElementState::Signal(SignalState::Ks1),
                ),
                TargetState(
                    "E".to_owned(),
                    TrackElement::Point,
                    TrackElementState::Point(PointState::Left),
                ),
                TargetState(
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
                TargetState(
                    "E".to_owned(),
                    TrackElement::Point,
                    TrackElementState::Point(PointState::Left),
                ),
                TargetState(
                    "G".to_owned(),
                    TrackElement::Signal,
                    TrackElementState::Signal(SignalState::Ks1),
                ),
                TargetState(
                    "H".to_owned(),
                    TrackElement::Signal,
                    TrackElementState::Signal(SignalState::Ks1),
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

    let generated = generate::generate(routes)?;

    let is_development = std::env::var(DEVELOPMENT_ENV).is_ok() || args.development;
    if is_development {
        println!("Running in dev mode");

        fs::create_dir_all("examples")?;

        let mut fp = std::fs::File::create("examples/dev_ixl.rs").unwrap_or_else(|_| {
            std::fs::OpenOptions::new()
                .write(true)
                .open("examples/dev_ixl.rs")
                .unwrap()
        });
        fp.write_all(generated.as_bytes())?;
    } else if let Some(output) = args.output {
        let mut fp = std::fs::File::create(output.clone()).unwrap_or_else(|_| {
            std::fs::OpenOptions::new()
                .write(true)
                .open(output)
                .unwrap()
        });
        fp.write_all(generated.as_bytes())?;
    } else {
        println!("{}", generated);
    }

    Ok(())
}
