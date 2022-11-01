mod Driveway;

pub struct Message {
    message: String,
}

impl Message {
    pub fn new(message: String) -> Self {
        Self { message }
    }

    fn print(&self) {
        println!("{}", self.message)
    }
}

pub trait TrackElement {
    type State;

    fn state(&self) -> &Self::State;
    fn set_state(&mut self, new_state: Self::State);
}

#[derive(Debug)]
pub enum PointState {
    Left,
    Right,
}

pub struct Point {
    state: PointState,
}

impl TrackElement for Point {
    type State = PointState;

    fn state(&self) -> &Self::State {
        &self.state
    }

    fn set_state(&mut self, new_state: Self::State) {
        self.state = new_state;
        println!("Point state is now {:?}", self.state);
    }

}
