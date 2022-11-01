use crate::TrackElement;

macro_rules! define_target_state {
    ($(($id:ident, $elem:ty, $state:ty)),+) => {
        pub struct TargetState {$($id: Vec<($elem, $state)>),+}

        impl TargetState {
            pub fn new($($id: Vec<($elem, $state)>),+) -> Self {
                Self {$($id),+}
            }

            pub fn set_state(&mut self) {
                $(for (e, s) in &mut self.$id {
                    e.set_state(*s);
                });+
            }
        }
    };
}

define_target_state!(
    (point, crate::point::Point, crate::point::PointState),
    (signal, crate::signal::Signal, crate::signal::SignalState)
);

pub struct Driveway {
    conflicting_driveways: Vec<String>,
    is_set: bool,
    target_state: TargetState,
}

impl Driveway {
    pub fn new(conflicting_driveways: Vec<String>, expected_state: TargetState) -> Self {
        Self {
            conflicting_driveways,
            is_set: false,
            target_state: expected_state,
        }
    }

    pub fn is_set(&self) -> bool {
        self.is_set
    }

    pub fn set_way(&mut self) -> bool {
        self.target_state.set_state();
        true
    }
}
