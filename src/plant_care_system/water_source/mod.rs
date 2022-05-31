
use super::device::Device;
use super::device::State;

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
       // PENDING:
       State::Off
    }
}

impl WaterSource
{
    pub fn new() -> WaterSource
    {
         WaterSource { _state: State::Off }
    }
}
