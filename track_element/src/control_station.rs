use std::io::Write;

use crate::{driveway::DrivewayManager, TrackElementError};

pub struct ControlStation {
    driveway_manager: DrivewayManager,
}

impl ControlStation {
    pub fn new(driveway_manager: DrivewayManager) -> Self {
        Self { driveway_manager }
    }

    pub fn set_driveway(
        &self,
        start_signal_id: &str,
        end_signal_id: &str,
    ) -> Result<(), TrackElementError> {
        self.driveway_manager
            .set_driveway(start_signal_id, end_signal_id)
    }

    pub fn start(&self) {
        let driveways = self.driveway_manager.get_driveway_ids();
        loop {
            println!("Existing Driveways: {driveways:?}");
            print!("> ");
            std::io::stdout().flush().unwrap();
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            let mut args = input.split_whitespace();
            let cmd = args.next().unwrap();
            match cmd {
                "set" => {
                    if let (Some(from), Some(to)) = (args.next(), args.next()) {
                        println!("Setting driveway from {from} to {to}");
                        if let Err(e) = self.driveway_manager.set_driveway(from, to) {
                            println!("An error occurred: {e:?}");
                        }
                    } else {
                        println!("Error: Please provide two valid signals.");
                    }
                }
                "help" => {
                    println!(
                        r#"==== HELP ====

set [from] [to]
    Sets the driveway between signals [from] and [to]

quit
    Exits this control station

help
    Shows this help
                    "#
                    )
                }
                "quit" => {
                    println!("Exiting control station");
                    break;
                }
                _ => {
                    println!("Sorry, command '{cmd}' is unknown");
                }
            }
        }
    }
}
