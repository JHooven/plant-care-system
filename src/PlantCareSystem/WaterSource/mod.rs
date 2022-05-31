
use super::Device::Device;
use super::Device::State;

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

