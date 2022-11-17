use std::rc::Rc;
use std::{borrow::Borrow, cell::RefCell};

use crate::{
    driveway::Driveway,
    driveway::TargetState,
    point::{Point, PointState},
    signal::{Signal, SignalState, SignalType},
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
fn set_basic_driveway() {
    let p1 = Rc::new(RefCell::new(Point::new(PointState::Left, "P1".to_string())));
    let p2 = Rc::new(RefCell::new(Point::new(PointState::Left, "P2".to_string())));
    let s = Rc::new(RefCell::new(Signal::new(
        SignalState::default(),
        SignalType::ToDo,
        "S".to_string(),
    )));

    let ts = TargetState::new(
        vec![
            (p1.clone(), PointState::Right),
            (p2.clone(), PointState::Left),
        ],
        vec![(s.clone(), SignalState::Ks1)],
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
            .state(),
        SignalState::Ks1
    ));
}
