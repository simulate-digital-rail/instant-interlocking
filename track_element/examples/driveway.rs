use std::rc::Rc;
use std::cell::RefCell;
use track_element::{
    TrackElement,
    driveway::{Driveway, TargetState},
    point::{Point, PointState},
};

fn main() {
    let p1 = Rc::new(RefCell::new(Point::new(PointState::Left, "A".to_owned())));
    let p2 = Rc::new(RefCell::new(Point::new(PointState::Left, "B".to_owned())));

    let ts = TargetState::new(
        vec![(p1.clone(), PointState::Right), (p2.clone(), PointState::Left)],
        vec![],
    );

    let mut dw = Driveway::new(Vec::new(), ts);
    dw.set_way().unwrap();
    assert!(matches!(p1.borrow().state(), PointState::Right));
    assert!(matches!(p2.borrow().state(), PointState::Left));
}
