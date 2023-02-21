use std::{io::Write, path::PathBuf};

use clap::Parser;
use driveway::{DrivewayRepr, TargetState, TrackElement, TrackElementState};
use track_element::{point::PointState, signal::MainSignalState};

mod driveway;
mod generate;

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
    #[arg(long, short, value_hint = clap::ValueHint::DirPath)]
    output: PathBuf,
    /// Use the example data provided by this tool (ignores JSON input)
    #[arg(long, short)]
    example: bool,
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

    let mut output_path = std::env::current_dir()?;
    output_path.push(&args.output);

    std::fs::create_dir_all(&output_path)?;

    std::process::Command::new("cargo")
        .current_dir(&output_path)
        .args(["init", "--name", "ixl"])
        .spawn()?
        .wait()?;

    let mut manifest_path = output_path.clone();
    manifest_path.push("Cargo.toml");

    let mut manifest = std::fs::OpenOptions::new()
        .append(true)
        .open(&manifest_path)?;
    manifest.write_all(b"\n[workspace]")?;

    std::process::Command::new("cargo")
        .current_dir(&output_path)
        .args(["add", "--path", "../track_element", "track_element"])
        .spawn()?
        .wait()?;

    let mut src_dir = output_path.clone();
    src_dir.push("src/main.rs");

    // Interlocking
    let mut fp = std::fs::File::create(&src_dir)?;
    fp.write_all(generated.as_bytes())?;

    // Tests
    let mut fp = std::fs::File::create(&src_dir.with_file_name("test.rs"))?;
    fp.write_all(generated_tests.as_bytes())?;

    Ok(())
}
