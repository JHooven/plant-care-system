
extern crate timer;
extern crate chrono;


mod plant_wattering_system;

use std::{thread, time::Duration};

use plant_wattering_system::plant_watering_system as pws;

use rand::Rng;

fn build_plant() -> pws::Plant
{
    let hygro: pws::Hygrometer = pws::Hygrometer::new();
    let mut vec_hygro: Vec<pws::Hygrometer> = Vec::<pws::Hygrometer>::new();
    vec_hygro.push(hygro);

    let water: pws::WaterSource = pws::WaterSource::new();
    let mut vec_water: Vec<pws::WaterSource> = Vec::<pws::WaterSource>::new();
    vec_water.push(water);

    let plant: pws::Plant = pws::Plant::new("Pothos1".to_string(), vec_hygro, vec_water);

    plant
}
fn main()  
{
    let mut plant_system : pws::PlantWatteringSystem = pws::PlantWatteringSystem::new();

    let mut plant : pws::Plant = build_plant();
    
    plant_system.addPlant(plant);

    plant_system.start();

    let mut my_timer : timer::Timer = timer::Timer::new();

    let _guard = my_timer.schedule_repeating(chrono::Duration::seconds(10), 
       move || {
        plant_system.checkUpdatePlants();
      });

    //plant_system.checkUpdatePlants();
    
    loop 
    {
        thread::sleep(Duration::from_secs(12));
    }
}