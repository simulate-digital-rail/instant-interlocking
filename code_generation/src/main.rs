use std::{io::Write, path::PathBuf};

use clap::Parser;
use driveway::{
    DrivewayRepr, MainSignalState, PointState, SignalState, SupportedSignalStates, TrackElement,
};

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
            start_signal: "A".to_owned(),
            end_signal: "C".to_owned(),
            states: vec![
                TrackElement::Signal {
                    uuid: "A".to_owned(),
                    name: None,
                    supported_states: SupportedSignalStates {
                        main: vec![MainSignalState("Hp0".into()), MainSignalState("Ks1".into())],
                        zs3: None,
                        zs3v: None,
                    },
                    state: SignalState {
                        main: MainSignalState("Ks1".into()),
                        zs3: None,
                        zs3v: None,
                    },
                },
                TrackElement::Point {
                    uuid: "B".to_owned(),
                    state: PointState::Left,
                },
                TrackElement::Signal {
                    uuid: "C".to_owned(),
                    name: None,
                    supported_states: SupportedSignalStates {
                        main: vec![MainSignalState("Hp0".into()), MainSignalState("Ks1".into())],
                        zs3: None,
                        zs3v: None,
                    },
                    state: SignalState {
                        main: MainSignalState("Ks1".into()),
                        zs3: None,
                        zs3v: None,
                    },
                },
            ],
        },
        DrivewayRepr {
            start_signal: "C".to_owned(),
            end_signal: "D".to_owned(),
            states: vec![
                TrackElement::Point {
                    uuid: "B".to_owned(),
                    state: PointState::Left,
                },
                TrackElement::Signal {
                    uuid: "C".to_owned(),
                    name: None,
                    supported_states: SupportedSignalStates {
                        main: vec![MainSignalState("Hp0".into()), MainSignalState("Ks1".into())],
                        zs3: None,
                        zs3v: None,
                    },
                    state: SignalState {
                        main: MainSignalState("Ks1".into()),
                        zs3: None,
                        zs3v: None,
                    },
                },
                TrackElement::Signal {
                    uuid: "D".to_owned(),
                    name: None,
                    supported_states: SupportedSignalStates {
                        main: vec![MainSignalState("Hp0".into()), MainSignalState("Ks1".into())],
                        zs3: None,
                        zs3v: None,
                    },
                    state: SignalState {
                        main: MainSignalState("Ks1".into()),
                        zs3: None,
                        zs3v: None,
                    },
                },
            ],
        },
        DrivewayRepr {
            start_signal: "D".to_owned(),
            end_signal: "F".to_owned(),
            states: vec![
                TrackElement::Signal {
                    uuid: "D".to_owned(),
                    name: None,
                    supported_states: SupportedSignalStates {
                        main: vec![MainSignalState("Hp0".into()), MainSignalState("Ks1".into())],
                        zs3: None,
                        zs3v: None,
                    },
                    state: SignalState {
                        main: MainSignalState("Ks1".into()),
                        zs3: None,
                        zs3v: None,
                    },
                },
                TrackElement::Point {
                    uuid: "E".to_owned(),
                    state: PointState::Left,
                },
                TrackElement::Signal {
                    uuid: "F".to_owned(),
                    name: None,
                    supported_states: SupportedSignalStates {
                        main: vec![MainSignalState("Hp0".into()), MainSignalState("Ks1".into())],
                        zs3: None,
                        zs3v: None,
                    },
                    state: SignalState {
                        main: MainSignalState("Ks1".into()),
                        zs3: None,
                        zs3v: None,
                    },
                },
            ],
        },
        DrivewayRepr {
            start_signal: "G".to_owned(),
            end_signal: "H".to_owned(),
            states: vec![
                TrackElement::Point {
                    uuid: "E".to_owned(),
                    state: PointState::Left,
                },
                TrackElement::Signal {
                    uuid: "G".to_owned(),
                    name: None,
                    supported_states: SupportedSignalStates {
                        main: vec![MainSignalState("Hp0".into()), MainSignalState("Ks1".into())],
                        zs3: None,
                        zs3v: None,
                    },
                    state: SignalState {
                        main: MainSignalState("Ks1".into()),
                        zs3: None,
                        zs3v: None,
                    },
                },
                TrackElement::Signal {
                    uuid: "H".to_owned(),
                    name: None,
                    supported_states: SupportedSignalStates {
                        main: vec![MainSignalState("Hp0".into()), MainSignalState("Ks1".into())],
                        zs3: None,
                        zs3v: None,
                    },
                    state: SignalState {
                        main: MainSignalState("Ks1".into()),
                        zs3: None,
                        zs3v: None,
                    },
                },
            ],
        },
    ];

    let routes: Vec<_> = if args.example {
        example_routes
    } else {
        serde_json::from_str(&std::fs::read_to_string(args.input.clone().unwrap())?)?
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
