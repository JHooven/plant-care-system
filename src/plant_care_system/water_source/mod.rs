
pub use super::device::Device;
pub use super::device::State;
mod unit_tests;

pub struct WaterSource
{
    pub name: String,
    pub state: State,
}

impl Device for WaterSource
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

impl WaterSource
{
    pub fn new(name: String) -> WaterSource
    {
         WaterSource {name, state: State::Off }
    }
}
