
#[allow(unused_variables, unused_assignments, unused_imports)]

extern crate rand;
extern crate timer;
extern crate chrono;

use std::sync::mpsc::channel;
use std::{thread, time::Duration};

use rand::Rng;

#[allow(unused_variables, unused_imports)]

pub mod plant_watering_system
{
    use rand::Rng;

    //Hygrometer
    pub struct Hygrometer
    {
        last_reading: f32,
    }

    impl Hygrometer
    {
        pub fn newReading(&mut self)
        {
            // PENDING: gpio code goes here
            let mut rng = rand::thread_rng();
            self.last_reading = rng.gen_range(0.0, 400.0);
        }

        pub fn reading(&self) -> f32
        {
            self.last_reading
        }

        pub fn new() -> Hygrometer
        {
            Hygrometer { last_reading: 0.0 }
        }
    }

    // WaterSource
    pub struct WaterSource
    {
        // True == on
        state: bool, 
    }

    impl WaterSource
    {
        fn turn_on( &mut self)
        {
            // PENDING: gpio code goes here.
            self.state = true;
        }

        fn turn_off( &mut self)
        {
            // PENDING: gpio code goes here.
            self.state = false;
        }

        pub fn new() -> WaterSource
        {
            WaterSource { state: false }
        }
    }

    // Plant
    pub struct Plant
    {
        name: String,
        hygrometers: Vec<Hygrometer>,
        water_sources: Vec<WaterSource>,
        hygro_low_watter_mark: f32,
        hygro_high_watter_mark: f32,
    }

    impl Plant
    {
        // Name of the plant
        pub fn name(&self) -> &String {&self.name}
        pub fn hygrometers(&self) -> &Vec<Hygrometer> {&self.hygrometers}
        pub fn water_sources(&self) -> &Vec<WaterSource> {&self.water_sources}
        pub fn hygro_avg(&self) -> f32 
        {
            let mut total: f32 = 0.0;

            for hygro in self.hygrometers.iter()
            {
                total += hygro.reading();
            } 

            total / (self.hygrometers.len() as f32)
        }

        pub fn new(_name: String, hygros: Vec<Hygrometer>, _water_sources: Vec<WaterSource> ) -> Plant
        {
            Plant
            {
                name: _name
                , hygrometers: hygros
                , water_sources: _water_sources
                , hygro_high_watter_mark: 400.0
                , hygro_low_watter_mark: 100.0
            }
        }

    }

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
                        if !water_source.state
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
                        if water_source.state
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

    // PlanWateringSystem
    pub struct PlantWatteringSystem
    {
        plant_controller: PlantController,
    }

    impl PlantWatteringSystem
    {
        pub fn checkUpdatePlants(&mut self)
        {
            self.plant_controller.update_plants();
        }

        pub fn start(&self)
        {
            // let mut timer : timer::Timer = timer::Timer::new();

            // let _guard = timer.schedule_with_delay(chrono::Duration::seconds(30), move || {
            
            // });
        }

        pub fn addPlant(&mut self, plant : Plant)
        {
            self.plant_controller.addPlant(plant);
        }

        pub fn new() -> PlantWatteringSystem
        {
            PlantWatteringSystem{plant_controller: PlantController::new()}
        }
    }
}

