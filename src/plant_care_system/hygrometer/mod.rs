extern crate rand;
extern crate timer;
extern crate chrono;

pub use super::device::Device;
pub use super::device::State;

mod unit_tests;

use rand::{Rng};

//Hygrometer
pub struct Hygrometer
{
    last_reading: f32,
    is_on: bool,
    _state: State,
}

impl Device for Hygrometer
{
    fn turn_on(&mut self)
    {
        // GPIO code goes here
        self._state = State::On;
    }
    
    fn turn_off(&mut self)
    {
        self._state = State::Off;
    }
    
    fn state(&self) -> State
    {
       self._state
    }
}

impl Hygrometer
{
    pub fn update_value(&mut self) -> f32
    {
        // PENDING: gpio code goes here
        let mut rng = rand::thread_rng();
        self.last_reading = rng.gen_range(0.0, 400.0);

        self.last_reading
    }

    pub fn last_value(&self) -> f32
    {
        self.last_reading
    }

    pub fn new() -> Hygrometer
    {
        Hygrometer { last_reading: 0.0, is_on: false, _state: State::Off}
    }
}