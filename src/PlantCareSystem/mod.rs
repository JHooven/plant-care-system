
#[allow(unused_variables, unused_assignments, unused_imports)]

pub mod PlantController;
pub mod Hygrometer;
pub mod WaterSource;
pub mod Plant;
pub mod Device;

extern crate rand;
extern crate timer;
extern crate chrono;

#[allow(unused_variables, unused_imports)]

pub mod PlantCareSystem
{
    use super::PlantController::PlantController;
    use super::Plant::Plant;

    // PlanWateringSystem
    pub struct PlantCareSystem
    {
        plant_controller: PlantController,
    }

    impl PlantCareSystem
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

        pub fn new() -> PlantCareSystem
        {
            PlantCareSystem{plant_controller: PlantController::new()}
        }
    }
}

