use super::time::Time;


pub struct World {
    pub time: Time
}


impl World {
    pub fn new() -> Self { World { time: Time::new() } }
}
