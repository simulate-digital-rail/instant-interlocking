pub mod driveway;
pub mod point;
pub mod signal;

#[cfg(test)]
mod test;

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
    type State: Copy;

    fn state(&self) -> Self::State;
    fn set_state(&mut self, new_state: Self::State);
}
