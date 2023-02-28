use std::sync::Arc;
use std::sync::RwLock;
use track_element::{
    driveway::{Driveway, DrivewayState},
    point::{Point, PointState},
    signal::{MainSignalState, Signal, SignalState, SupportedSignalStates},
    TrackElement,
};

fn main() {
    let p1 = Arc::new(RwLock::new(Point::new(PointState::Left, "P1".to_string())));
    let p2 = Arc::new(RwLock::new(Point::new(PointState::Left, "P2".to_string())));
    let s = Arc::new(RwLock::new(Signal::new(
        SignalState::default(),
        SupportedSignalStates::default()
            .main(&mut vec![MainSignalState::Hp0, MainSignalState::Ks1]),
        "S".to_string(),
        None,
    )));

    let ts = DrivewayState::new(
        vec![
            (p1.clone(), PointState::Right),
            (p2.clone(), PointState::Left),
        ],
        vec![(s.clone(), (MainSignalState::Ks1).into())],
        vec![],
    );

    let mut dw = Driveway::new(Vec::new(), ts, s.clone(), s.clone());
    dw.set_way().unwrap();
    assert!(matches!(p1.read().unwrap().state(), PointState::Right));
    assert!(matches!(p2.read().unwrap().state(), PointState::Left));
    assert!(matches!(
        s.read().unwrap().state().main(),
        MainSignalState::Ks1
    ));
}
