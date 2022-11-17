use std::cell::RefCell;
use std::rc::Rc;

use crate::{
    point::{Point, PointState},
    TrackElement, driveway::TargetState, driveway::Driveway, signal::{Signal, SignalState, SignalType},
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
    let mut s = Signal::new(SignalState::Hp0, SignalType::ToDo, "A".to_string());    
    assert!(matches!(s.state(), SignalState::Hp0));
    s.set_state(SignalState::Ks1).unwrap();
    assert!(matches!(s.state(), SignalState::Ks1));
}

#[test]
fn set_basic_driveway() {
    let p1 = Rc::new(RefCell::new(Point::new(PointState::Left, String::new())));
    let p2 = Rc::new(RefCell::new(Point::new(PointState::Left, String::new())));
    let s1 = Rc::new(RefCell::new(Signal::new(SignalState::Hp0, SignalType::ToDo, "A".to_string())));
    let s2 = Rc::new(RefCell::new(Signal::new(SignalState::Hp0, SignalType::ToDo, "B".to_string())));
    
    let ts = TargetState::new(
        vec![(p1.clone(), PointState::Right), (p2.clone(), PointState::Left)],
        vec![],
    );
    
    let mut dw = Driveway::new(Vec::new(), ts, s1, s2);
    dw.set_way().unwrap();
    assert!(matches!(p1.borrow().state(), PointState::Right));
    assert!(matches!(p2.borrow().state(), PointState::Left));
}

#[test]
fn set_conflicting_driveway(){
    let s1 = Rc::new(RefCell::new(Signal::new(SignalState::Hp0, SignalType::ToDo, "A".to_string())));
    let s2 = Rc::new(RefCell::new(Signal::new(SignalState::Hp0, SignalType::ToDo, "B".to_string())));
    let s12 = Rc::new(RefCell::new(Signal::new(SignalState::Hp0, SignalType::ToDo, "C".to_string())));
    let s22 = Rc::new(RefCell::new(Signal::new(SignalState::Hp0, SignalType::ToDo, "D".to_string())));

    let dw1 = Rc::new(RefCell::new(Driveway::new(Vec::new(), TargetState::new(Vec::new(), Vec::new()), s1, s2)));
    let mut dw2 = Driveway::new(vec![dw1.clone()], TargetState::new(Vec::new(), Vec::new()), s12, s22);

    dw1.borrow_mut().set_way().unwrap();
    assert!(dw2.set_way().is_err())
}
