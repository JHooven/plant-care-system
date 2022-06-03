
#[cfg(test)]

extern crate serde;
extern crate serde_json;

use serde::{Serialize, Deserialize};

use crate::plant_care_system;
use crate::plant_care_system::hygrometer as hyg;
use crate::plant_care_system::hygrometer::Hygrometer;
use hyg::Device;
use hyg::State;

#[test]
fn hydrometer_new_test()
{
    let hygro : hyg::Hygrometer = hyg::Hygrometer::new("Hygro_1".to_string());

    let s : hyg::State = hygro.state();
    
    assert_eq!(s, State::Off);
}

#[test]
fn hygrometer_turn_on_test()
{
    let mut hygro : hyg::Hygrometer = hyg::Hygrometer::new("Hygro_1".to_string());

    let mut s : State = hygro.state();
    
    assert_eq!(s, State::Off);

    hygro.turn_on();

    s = hygro.state();

    assert_eq!(s, State::On);        
}

#[test]
fn hygrometer_turn_off_test()
{
    let mut hygro : hyg::Hygrometer = hyg::Hygrometer::new("Hygro_1".to_string());

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
    let mut hygro : hyg::Hygrometer = hyg::Hygrometer::new("Hygro_1".to_string());

    dbg!("hygro.value(): {}", hygro.last_value());
    assert_ne!(hygro.update_value(), 0.0);
    dbg!("new hygro.value(): {}", hygro.last_value());
}   

#[test]
fn hygrometer_serialization_test()
{
    let mut hygro1 = hyg::Hygrometer::new("Hygro_1".to_string());

    assert_ne!(hygro1.update_value(), 0.0);

    let serialized = serde_json::to_string(&hygro1).unwrap();

    dbg!("hygro json: {}", &serialized);

    let hygro2: Hygrometer = serde_json::from_str(&serialized.to_string()).unwrap();

    assert_eq!(hygro1.last_reading, hygro2.last_reading);
    
}
