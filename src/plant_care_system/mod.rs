
#[allow(dead_code, unused_variables, unused_assignments, unused_imports)]

pub mod plant_controller;
pub mod hygrometer;
pub mod water_source;
pub mod plant;
pub mod device;

extern crate rand;
extern crate timer;
extern crate chrono;

#[allow(unused_variables, unused_imports)]

pub mod plant_care_system
{
    use super::plant_controller::PlantController;
    use super::plant::Plant;

    // PlanWateringSystem
    pub struct PlantCareSystem
    {
        plant_controller: PlantController,
    }

    impl PlantCareSystem
    {
        pub fn check_update_plants(&mut self)
        {
            self.plant_controller.update_plants();
        }

        pub fn start(&self)
        {
            // let mut timer : timer::Timer = timer::Timer::new();

            // let _guard = timer.schedule_with_delay(chrono::Duration::seconds(30), move || {
            
            // });
        }

        pub fn add_plant(&mut self, plant : Plant)
        {
            self.plant_controller.add_plant(plant);
        }

        pub fn new() -> PlantCareSystem
        {
            PlantCareSystem{plant_controller: PlantController::new()}
        }
    }
}

