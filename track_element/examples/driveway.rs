use track_element::{
    driveway::{Driveway, TargetState},
    point::{Point, PointState},
};

fn main() {
    let p1 = Point::new(PointState::Left);
    let p2 = Point::new(PointState::Left);

    let ts = TargetState::new(
        vec![(p1, PointState::Right), (p2, PointState::Left)],
        vec![],
    );

    let mut dw = Driveway::new(Vec::new(), ts);
    dw.set_way();
}
