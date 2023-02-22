use signal::MainSignalState;

pub mod control_station;
pub mod driveway;
pub mod point;
pub mod signal;

#[cfg(test)]
mod test;
pub mod vacancy_section;

#[derive(Debug)]
pub enum TrackElementError {
    DrivewayDoesNotExist(String),
    HasConflictingDriveways,
    InvalidAdditionalSignalState,
    InvalidMainSignalState(MainSignalState),
}

impl std::fmt::Display for TrackElementError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for TrackElementError {}

pub trait TrackElement {
    type State: Copy + Default;

    fn id(&self) -> &str;
    fn state(&self) -> Self::State;
    fn set_state(&mut self, new_state: Self::State) -> Result<(), TrackElementError>;
}
