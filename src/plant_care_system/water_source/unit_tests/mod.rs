#[cfg(test)]

use crate::plant_care_system;
use crate::plant_care_system::water_source as pws;
use pws::Device;
use pws::State;

#[test]
fn water_source_new_test()
{
    let p : pws::WaterSource =  pws::WaterSource::new();

    let s : pws::State = p.state();
    
    assert_eq!(s, State::Off);
}

#[test]
fn water_source_turn_on_test()
{
    let mut p : pws::WaterSource = pws::WaterSource::new();

    let mut s : State = p.state();
    
    assert_eq!(s, State::Off);

    p.turn_on();

    s = p.state();

    assert_eq!(s, State::On);        
}

#[test]
fn water_source_turn_off_test()
{
    let mut p : pws::WaterSource = pws::WaterSource::new();

    let mut s : State = p.state();
    
    assert_eq!(s, State::Off);

    p.turn_on();

    s = p.state();

    assert_eq!(s, State::On);   
    
    p.turn_off();

    s = p.state();
    
    assert_eq!(s, State::Off);
}
