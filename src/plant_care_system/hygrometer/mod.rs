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
    pub name : String,
    pub last_reading: f32,
    pub is_on: bool,
    pub state: State,
}

impl Device for Hygrometer
{
    fn turn_on(&mut self)
    {
        // GPIO code goes here
        self.state = State::On;
    }
    
    fn turn_off(&mut self)
    {
        self.state = State::Off;
    }
    
    fn state(&self) -> State
    {
       self.state
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

    pub fn new(_name : String) -> Hygrometer
    {
        Hygrometer { name: _name, last_reading: 0.0, is_on: false, state: State::Off}
    }
}