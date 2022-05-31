
use crate::PlantCareSystem::Device::Device;
use crate::PlantCareSystem::Device::State;
use super::Plant::Plant;

pub struct PlantController
{
    plants: Vec<Plant>,   
    update_interval: i32,
}

impl PlantController 
{
    pub fn start(&self)
    {

    }
    pub fn addPlant(&mut self, plant: Plant)
    {
        self.plants.push(plant);
    }

    pub fn update_plants(&mut self)
    {
        println!("Updating plant states.");

        for plant in self.plants.iter_mut()
        {
            println!("Checking plant: {}", plant.name());

            // Check if the plant needs watering
            if plant.hygro_avg() <= plant.hygro_low_watter_mark
            {
                println!("Plant: {} hygro_avg: {} is below the low water mark.", plant.name(), plant.hygro_avg());
                
                for water_source in plant.water_sources.iter_mut()
                {
                    if water_source.state() != State::Off
                    {
                         water_source.turn_on();
                    }
                }
            }
            else if plant.hygro_avg() >= plant.hygro_high_watter_mark
            {
                println!("Plant: {} hygro_avg: {} is below the high water mark.", plant.name(), plant.hygro_avg());

                for water_source in plant.water_sources.iter_mut()
                {
                    if water_source.state() != State::On
                    {
                        water_source.turn_off();
                    }
                }
            }
        } 
    } 
    
    pub fn new() -> PlantController
    {
        let v : Vec<Plant> = Vec::new();
        
        PlantController { plants: v, update_interval: 5 }
    }
}