use std::rc::Rc;
use std::{borrow::Borrow, cell::RefCell};

use crate::signal::{MainSignalState, SupportedSignalStates};
use crate::{
    driveway::Driveway,
    driveway::TargetState,
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
    );
    assert!(matches!(s.state().main(), MainSignalState::Hp0));
    s.set_state((MainSignalState::Ks1).into()).unwrap();
    assert!(matches!(s.state().main(), MainSignalState::Ks1));
}

#[test]
fn set_basic_driveway() {
    let p1 = Rc::new(RefCell::new(Point::new(PointState::Left, "P1".to_string())));
    let p2 = Rc::new(RefCell::new(Point::new(PointState::Left, "P2".to_string())));
    let s = Rc::new(RefCell::new(Signal::new(
        SignalState::default(),
        SupportedSignalStates::default()
            .main(&mut vec![MainSignalState::Hp0, MainSignalState::Ks1]),
        "S".to_string(),
    )));

    let ts = TargetState::new(
        vec![
            (p1.clone(), PointState::Right),
            (p2.clone(), PointState::Left),
        ],
        vec![(s.clone(), (MainSignalState::Ks1).into())],
    );

    let mut dw = Driveway::new(Vec::new(), ts, s.clone(), s.clone());
    dw.set_way().unwrap();

    // These types are only needed in test cases like this - They do not appear in the actual generated code.
    assert!(matches!(
        <Rc<RefCell<Point>> as Borrow<RefCell<Point>>>::borrow(&p1)
            .borrow()
            .state(),
        PointState::Right
    ));
    assert!(matches!(
        <Rc<RefCell<Point>> as Borrow<RefCell<Point>>>::borrow(&p2)
            .borrow()
            .state(),
        PointState::Left
    ));
    assert!(matches!(
        <Rc<RefCell<Signal>> as Borrow<RefCell<Signal>>>::borrow(&s)
            .borrow()
            .state()
            .main(),
        MainSignalState::Ks1
    ));
}

#[test]
fn set_conflicting_driveway() {
    let s1 = Rc::new(RefCell::new(Signal::new(
        (MainSignalState::Hp0).into(),
        SupportedSignalStates::default()
            .main(&mut vec![MainSignalState::Hp0, MainSignalState::Ks1]),
        "A".to_string(),
    )));
    let s2 = Rc::new(RefCell::new(Signal::new(
        (MainSignalState::Hp0).into(),
        SupportedSignalStates::default()
            .main(&mut vec![MainSignalState::Hp0, MainSignalState::Ks1]),
        "B".to_string(),
    )));
    let s12 = Rc::new(RefCell::new(Signal::new(
        (MainSignalState::Hp0).into(),
        SupportedSignalStates::default()
            .main(&mut vec![MainSignalState::Hp0, MainSignalState::Ks1]),
        "C".to_string(),
    )));
    let s22 = Rc::new(RefCell::new(Signal::new(
        (MainSignalState::Hp0).into(),
        SupportedSignalStates::default()
            .main(&mut vec![MainSignalState::Hp0, MainSignalState::Ks1]),
        "D".to_string(),
    )));

    let dw1 = Rc::new(RefCell::new(Driveway::new(
        Vec::new(),
        TargetState::new(Vec::new(), Vec::new()),
        s1,
        s2,
    )));
    let mut dw2 = Driveway::new(
        vec![dw1.clone()],
        TargetState::new(Vec::new(), Vec::new()),
        s12,
        s22,
    );

    dw1.borrow_mut().set_way().unwrap();
    assert!(dw2.set_way().is_err())
}
