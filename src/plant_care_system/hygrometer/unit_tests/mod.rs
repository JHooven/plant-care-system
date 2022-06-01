#[cfg(test)]

use crate::plant_care_system;
use crate::plant_care_system::hygrometer as hyg;
use hyg::Device;
use hyg::State;

#[test]
fn hydrometer_new_test()
{
    let hygro : hyg::Hygrometer =  hyg::Hygrometer::new();

    let s : hyg::State = hygro.state();
    
    assert_eq!(s, State::Off);
}

#[test]
fn hygrometer_turn_on_test()
{
    let mut hygro : hyg::Hygrometer = hyg::Hygrometer::new();

    let mut s : State = hygro.state();
    
    assert_eq!(s, State::Off);

    hygro.turn_on();

    s = hygro.state();

    assert_eq!(s, State::On);        
}

#[test]
fn hygrometer_turn_off_test()
{
    let mut hygro : hyg::Hygrometer = hyg::Hygrometer::new();

    let mut s : State = hygro.state();
    
    assert_eq!(s, State::Off);

    hygro.turn_on();

    s = hygro.state();

    assert_eq!(s, State::On);   
    
    hygro.turn_off();

    s = hygro.state();
    
    assert_eq!(s, State::Off);
}

#[test]
fn hygrometer_update_value_test()
{
    let mut hygro : hyg::Hygrometer = hyg::Hygrometer::new();

    dbg!("hygro.value(): {}", hygro.last_value());
    assert_ne!(hygro.update_value(), 0.0);
    dbg!("new hygro.value(): {}", hygro.last_value());
}   