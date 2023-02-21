use std::cell::RefCell;
use std::rc::Rc;
use track_element::{
    driveway::{Driveway, DrivewayState},
    point::{Point, PointState},
    signal::{MainSignalState, Signal, SignalState, SupportedSignalStates},
    TrackElement,
};

fn main() {
    let p1 = Rc::new(RefCell::new(Point::new(PointState::Left, "P1".to_string())));
    let p2 = Rc::new(RefCell::new(Point::new(PointState::Left, "P2".to_string())));
    let s = Rc::new(RefCell::new(Signal::new(
        SignalState::default(),
        SupportedSignalStates::default()
            .main(&mut vec![MainSignalState::Hp0, MainSignalState::Ks1]),
        "S".to_string(),
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
    assert!(matches!(p1.borrow().state(), PointState::Right));
    assert!(matches!(p2.borrow().state(), PointState::Left));
    assert!(matches!(s.borrow().state().main(), MainSignalState::Ks1));
}
