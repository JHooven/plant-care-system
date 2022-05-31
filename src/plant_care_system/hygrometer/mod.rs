extern crate rand;
extern crate timer;
extern crate chrono;

use rand::{Rng};

//Hygrometer
pub struct Hygrometer
{
    last_reading: f32,
    is_on: bool,
}

impl Hygrometer
{
    pub fn new_reading(&mut self)
    {
        // PENDING: gpio code goes here
        let mut rng = rand::thread_rng();
        self.last_reading = rng.gen_range(0.0, 400.0);
    }

    pub fn reading(&self) -> f32
    {
        self.last_reading
    }

    pub fn new() -> Hygrometer
    {
        Hygrometer { last_reading: 0.0, is_on: false}
    }
}