#[cfg(test)]

extern crate serde;
extern crate serde_json;

use serde::{Serialize, Deserialize};

use crate::plant_care_system;
use crate::plant_care_system::plant::{Plant, Hygrometer, WaterSource};
use crate::plant_care_system::device::{State, Device};

#[test]
fn plant_new_test()
{
    let hygro1 = Hygrometer::new("Hygro_1".to_string());
    let ws1 = WaterSource::new("Water_1".to_string());

    // Make a plant.
    let mut plant = Plant::new("Hooville Plant Controller 1".to_string());

    plant.hygrometers.push(hygro1);

    plant.water_sources.push(ws1);

    assert_eq!(plant.name, "Hooville Plant Controller 1".to_string());

    assert_eq!(plant.hygrometers().len(), 1);

}
