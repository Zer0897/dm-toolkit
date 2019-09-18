use super::time::Time;


pub struct World {
    pub time: Time
}


impl World {
    pub fn new() -> Self {
        Self { time: Time::new() }
    }
}
