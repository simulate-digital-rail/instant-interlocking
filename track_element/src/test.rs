use std::cell::RefCell;
use std::rc::Rc;

use crate::{
    point::{Point, PointState},
    TrackElement, driveway::TargetState, driveway::Driveway,
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
    let p1 = Rc::new(RefCell::new(Point::new(PointState::Left, String::new())));
    let p2 = Rc::new(RefCell::new(Point::new(PointState::Left, String::new())));

    let ts = TargetState::new(
        vec![(p1.clone(), PointState::Right), (p2.clone(), PointState::Left)],
        vec![],
    );

    let mut dw = Driveway::new(Vec::new(), ts);
    dw.set_way().unwrap();
    assert!(matches!(p1.borrow().state(), PointState::Right));
    assert!(matches!(p2.borrow().state(), PointState::Left));
}
