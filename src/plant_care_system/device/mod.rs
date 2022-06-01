#[allow(dead_code, unused_variables, unused_assignments, unused_imports)]

#[derive(PartialEq)]
#[derive(Debug)]
#[derive(Clone)]
#[derive(Copy)]
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