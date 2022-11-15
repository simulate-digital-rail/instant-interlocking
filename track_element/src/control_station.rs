use crate::driveway::DrivewayManager;

struct ControlStation {
    driveway_manager: DrivewayManager,
}

impl ControlStation {
    pub fn new(driveway_manager: DrivewayManager) -> Self {
        Self { driveway_manager }
    }

    pub fn set_driveway(&self, start_signal_id: &str, end_signal_id: &str) {
        self
            .driveway_manager.set_driveway(start_signal_id, end_signal_id);
    }
}
