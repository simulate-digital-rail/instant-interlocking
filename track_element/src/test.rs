use crate::{
    point::{Point, PointState},
    TrackElement,
};

#[test]
fn set_point() {
    let mut p = Point::new(PointState::Left);
    assert!(matches!(p.state(), PointState::Left));
    p.set_state(PointState::Right);
    assert!(matches!(p.state(), PointState::Right));
}
