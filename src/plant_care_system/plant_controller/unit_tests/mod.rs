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

    // Instantiate a Hygrometers Vec for constructing the Plant
    let mut hygros : Vec<Hygrometer> = Vec::<Hygrometer>::new();
    
    // Instantiate a Hygrometer for the hygros Vec 
    let hygro : Hygrometer = Hygrometer
    {
          name: "Plant_1".to_string()
        , last_reading: 0.0
        , is_on: false
        , state: State::Off
    };

    hygros.push(hygro);
    
    // Instantiate a WaterSources Vec for constructing the Plant
    let mut water_srcs : Vec<WaterSource> = Vec::<WaterSource>::new();

    // Instantiate a WaterSource for the water_srcs Vec
    let water_src  = WaterSource
    {
          name: "WS_1".to_string()
        , state: State::Off
    };

    water_srcs.push(water_src);

    // Instantiate the Plant!!!
    let plant = Plant
    {
         name: "Hooville Plant Controller 1".to_string()
        , hygrometers: hygros
        , water_sources: water_srcs
        , hygro_high_watter_mark: 400.0
        , hygro_low_watter_mark: 25.0
    };

    // Add the Plant to the PlantController
    pctl.add_plant(plant);

    // Get the Plant.name from the PlantController
    let _name: &String = &pctl.plants[0].name;

    // Check it.
    assert_eq!(_name, "Hooville Plant Controller 1");

    // Update outputs for all plant devices (Hygrometers, WaterSources, etc...)
    pctl.update_plants();

    let avg : f32 = pctl.plants[0].hygro_avg();

    assert!( avg > 0.0);

}

