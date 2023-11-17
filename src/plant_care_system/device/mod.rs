extern crate serde;
extern crate serde_json;

use serde::{Deserialize, Serialize};

//#[allow(dead_code, unused_variables, unused_assignments, unused_imports)]

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Copy)]
pub enum State {
    Unknown,
    Starting,
    On,
    Off,
    Break,
}

pub trait Device {
    fn turn_on(&mut self);
    fn turn_off(&mut self);
    fn state(&self) -> State;
}
