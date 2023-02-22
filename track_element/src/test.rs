use std::sync::{Arc, RwLock};

use crate::signal::{MainSignalState, SupportedSignalStates};
use crate::{
    driveway::Driveway,
    driveway::DrivewayState,
    point::{Point, PointState},
    signal::{Signal, SignalState},
    TrackElement,
};

#[test]
fn set_point() {
    let mut p = Point::new(PointState::Left, String::new());
    assert!(matches!(p.state(), PointState::Left));
    p.set_state(PointState::Right).unwrap();
    assert!(matches!(p.state(), PointState::Right));
}
#[test]
fn set_signal() {
    let mut s = Signal::new(
        (MainSignalState::Hp0).into(),
        SupportedSignalStates::default()
            .main(&mut vec![MainSignalState::Hp0, MainSignalState::Ks1]),
        "A".to_string(),
        None,
    );
    assert!(matches!(s.state().main(), MainSignalState::Hp0));
    s.set_state((MainSignalState::Ks1).into()).unwrap();
    assert!(matches!(s.state().main(), MainSignalState::Ks1));
}

#[test]
fn set_basic_driveway() {
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

    let mut dw = Driveway::new(Vec::new(), ts, "S".to_string(), "S".to_string());
    dw.set_way().unwrap();

    assert!(matches!(p1.read().unwrap().state(), PointState::Right));
    assert!(matches!(p2.read().unwrap().state(), PointState::Left));
    assert!(matches!(
        s.read().unwrap().state().main(),
        MainSignalState::Ks1
    ));
}

#[test]
fn set_conflicting_driveway() {
    let s1 = Arc::new(RwLock::new(Signal::new(
        (MainSignalState::Hp0).into(),
        SupportedSignalStates::default()
            .main(&mut vec![MainSignalState::Hp0, MainSignalState::Ks1]),
        "A".to_string(),
        None,
    )));
    let s2 = Arc::new(RwLock::new(Signal::new(
        (MainSignalState::Hp0).into(),
        SupportedSignalStates::default()
            .main(&mut vec![MainSignalState::Hp0, MainSignalState::Ks1]),
        "B".to_string(),
        None,
    )));
    let s12 = Arc::new(RwLock::new(Signal::new(
        (MainSignalState::Hp0).into(),
        SupportedSignalStates::default()
            .main(&mut vec![MainSignalState::Hp0, MainSignalState::Ks1]),
        "C".to_string(),
        None,
    )));
    let s22 = Arc::new(RwLock::new(Signal::new(
        (MainSignalState::Hp0).into(),
        SupportedSignalStates::default()
            .main(&mut vec![MainSignalState::Hp0, MainSignalState::Ks1]),
        "D".to_string(),
        None,
    )));

    let dw1 = Arc::new(RwLock::new(Driveway::new(
        Vec::new(),
        DrivewayState::new(
            Vec::new(),
            vec![
                (s1, (MainSignalState::Ks1).into()),
                (s2, (MainSignalState::Ks1).into()),
            ],
            Vec::new(),
        ),
        "A".to_string(),
        "B".to_string(),
    )));
    let mut dw2 = Driveway::new(
        vec![dw1.clone()],
        DrivewayState::new(
            Vec::new(),
            vec![
                (s12, (MainSignalState::Ks1).into()),
                (s22, (MainSignalState::Ks1).into()),
            ],
            Vec::new(),
        ),
        "C".to_string(),
        "D".to_string(),
    );

    dw1.write().unwrap().set_way().unwrap();
    assert!(dw2.set_way().is_err())
}
