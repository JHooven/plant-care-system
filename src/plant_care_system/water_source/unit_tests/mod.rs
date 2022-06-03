#[cfg(test)]

use crate::plant_care_system;
use crate::plant_care_system::water_source as pws;
use crate::plant_care_system::water_source::WaterSource;
use pws::Device;
use pws::State;

#[test]
fn water_source_new_test()
{
    let p : pws::WaterSource =  pws::WaterSource
    {
          name: "WS_1".to_string()
        , state: State::Off
    };

    let s : pws::State = p.state();
    
    assert_eq!(s, State::Off);
}

#[test]
fn water_source_turn_on_test()
{
    let mut p : pws::WaterSource = pws::WaterSource
    {
        name: "WS_1".to_string(),
        state: State::Off
    };

    let mut s : State = p.state();
    
    assert_eq!(s, State::Off);

    p.turn_on();

    s = p.state();

    assert_eq!(s, State::On);        
}

#[test]
fn water_source_turn_off_test()
{
    let mut p : pws::WaterSource = pws::WaterSource
    {
          name: "WS_1".to_string()
        , state: State::Off
    };

    let mut s : State = p.state();
    
    assert_eq!(s, State::Off);

    p.turn_on();

    s = p.state();

    assert_eq!(s, State::On);   
    
    p.turn_off();

    s = p.state();
    
    assert_eq!(s, State::Off);
}

#[test]
fn water_source_serialization_test()
{
    let mut ws1 = pws::WaterSource::new("Water_1".to_string());

    ws1.turn_on();
    assert_eq!(ws1.state(), State::On);

    let serialized = serde_json::to_string(&ws1).unwrap();

    dbg!("ws1 json: {}", &serialized);

    let ws2: WaterSource = serde_json::from_str(&serialized.to_string()).unwrap();

    assert_eq!(ws1.state(), ws2.state());
    
}
