#[cfg(test)]

use crate::plant_care_system as pcs;
use crate::plant_care_system::
{   
      device::{Device, State}
    , plant_controller as plc
    , hygrometer::Hygrometer
    , water_source::WaterSource
    , plant::{Plant}
};


#[test]
fn plant_controller_hygro_avg_test()
{
    // Instantiate a PlantController
    let mut pctl : plc::PlantController = plc::PlantController::new();
    
    // Instantiate a Hygrometer for the hygros Vec 
    let hygro : Hygrometer = Hygrometer::new("Plant_1".to_string());
    
    // Instantiate a WaterSource for the water_srcs Vec
    let water_src  = WaterSource::new("WS_1".to_string());
   
    // Instantiate the Plant!!!
    let mut plant = Plant::new("Hooville Plant 1".to_string());

    plant.hygrometers.push(hygro);

    plant.water_sources.push(water_src);

    // Add the Plant to the PlantController
    pctl.add_plant(plant);

    // Get the Plant.name from the PlantController
    let _name: &String = &pctl.plants[0].name;

    // Check it.
    assert_eq!(_name, &"Hooville Plant 1".to_string());

    // Update outputs for all plant devices (Hygrometers, WaterSources, etc...)
    pctl.update_plants();

    let avg : f32 = pctl.plants[0].hygro_avg();

    assert!( avg > 0.0);

}

