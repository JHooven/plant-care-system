
pub use super::device::Device;
pub use super::device::State;
mod unit_tests;

pub struct WaterSource
{
    _state: State,
}

impl Device for WaterSource
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

impl WaterSource
{
    pub fn new() -> WaterSource
    {
         WaterSource { _state: State::Off }
    }
}
