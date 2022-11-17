use std::cell::RefCell;
use std::rc::Rc;
use track_element::{
    driveway::{Driveway, TargetState},
    point::{Point, PointState},
    signal::{Signal, SignalState, SignalType},
    TrackElement,
};

fn main() {
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
    assert!(matches!(p1.borrow().state(), PointState::Right));
    assert!(matches!(p2.borrow().state(), PointState::Left));
    assert!(matches!(s.borrow().state(), SignalState::Ks1));
}
