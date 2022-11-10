struct ControlStation {
    driveways: Vec<&str>,
}

impl ControlStation {
    pub fn new(driveways: Vec<&str>) -> Self {
        Self { driveways }
    }

    pub fn set_driveway(&self, startSignal: &str, endSignal: &str) {
        let driveway = self
            .driveways
            .iter()
            .filter(|driveway| driveway[0] == startSignal && driveway[-1] == endSignal);
    }
}
