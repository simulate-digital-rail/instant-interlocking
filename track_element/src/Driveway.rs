
use crate::TrackElement;

// Driveway<((Point, PointState), (Signal, SignalState))>

pub struct Driveway<TargetState>{
    conflicting_driveways: Vec<String>,
    is_set: bool,
    expected_state: TargetState,
}

impl <TargetState> Driveway< TargetState> {
    pub fn set_way() -> bool {
        todo!()
    }
}
