use std::rc::Rc;
use std::cell::RefCell;
use track_element::{
    TrackElement,
    driveway::{Driveway, TargetState},
    point::{Point, PointState}, signal::{Signal, SignalState, SignalType},
};

fn main() {
    let p1 = Rc::new(RefCell::new(Point::new(PointState::Left, "A".to_owned())));
    let p2 = Rc::new(RefCell::new(Point::new(PointState::Left, "B".to_owned())));
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
