#[derive(PartialEq)]
pub enum State 
    {
        Unknown,
        Starting,
        On,
        Off,
        Break
    }

pub trait Device 
{
    fn turn_on(&mut self);
    fn turn_off(&mut self);
    fn state(&self) -> State;
}