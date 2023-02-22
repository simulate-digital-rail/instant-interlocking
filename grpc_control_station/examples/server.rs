use std::{
    collections::BTreeMap,
    sync::{Arc, RwLock},
};

use grpc_control_station::ControlStation;
use track_element::{
    driveway::{Driveway, DrivewayManager, DrivewayState},
    point::{Point, PointState},
    signal::{MainSignalState, Signal, SignalState, SupportedSignalStates},
    TrackElement,
};

#[tokio::main]
async fn main() {
    let p1 = Arc::new(RwLock::new(Point::new(PointState::Left, "P1".to_string())));
    let p2 = Arc::new(RwLock::new(Point::new(PointState::Left, "P2".to_string())));
    let s = Arc::new(RwLock::new(Signal::new(
        SignalState::default(),
        SupportedSignalStates::default()
            .main(&mut vec![MainSignalState::Hp0, MainSignalState::Ks1]),
        "A".to_string(),
        None,
    )));
    let s2 = Arc::new(RwLock::new(Signal::new(
        SignalState::default(),
        SupportedSignalStates::default()
            .main(&mut vec![MainSignalState::Hp0, MainSignalState::Ks1]),
        "N3".to_string(),
        None,
    )));

    let ts = DrivewayState::new(
        vec![
            (p1.clone(), PointState::Right),
            (p2.clone(), PointState::Left),
        ],
        vec![
            (s.clone(), (MainSignalState::Ks1).into()),
            (s2.clone(), (MainSignalState::Hp0).into()),
        ],
        vec![],
    );

    let dw = Arc::new(RwLock::new(Driveway::new(
        Vec::new(),
        ts,
        s.read().unwrap().id().into(),
        s2.read().unwrap().id().into(),
    )));
    let mut dwm = DrivewayManager::new(BTreeMap::new());
    dwm.add(dw);

    let mut control_station = ControlStation::new(dwm, "{}", "{}");

    let addr = "127.0.0.1:6006".parse().unwrap();
    control_station.listen(addr).await.unwrap();
}
