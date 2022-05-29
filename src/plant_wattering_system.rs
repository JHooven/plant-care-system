mod plant_watering_system
{
    //Hygrometer
    pub struct Hygrometer
    {
        reading: f32,
    }

    impl Hygrometer
    {
        pub fn reading() -> f32
        {
            1234.12
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
        // PENDING: gpio code goes here.
        fn turn_on( &mut self){self.state = true;}
        fn turn_off( &mut self){self.state = false;}
    }

    // Plant
    pub struct Plant
    {
        name: String,
        hygrometers: Vec<Hygrometer>,
        water_sources: Vec<WaterSource>,
        hygro_low_watter_mark: u32,
        hygro_high_watter_mark: u32,
    }

    impl Plant
    {
        // Name of the plant
        fn name(&self) -> &String {&self.name}
        fn hygrometers(&self) -> &Vec<Hygrometer> {&self.hygrometers}
        fn water_sources(&self) -> &Vec<WaterSource> {&self.water_sources}
        fn foo(&self) -> f32 {123.456}
        fn hygro_avg(&self) -> f32 
        {
            let total: f32 = 0.0;
            for hygro in self.hygrometers
            {
                total += hygro.reading();
            } 

            total / (self.hygrometers.len() as f32)
            
        }
    }

    pub struct PlantController
    {
        plants: Vec<Plant>,   
    }

    impl PlantController 
    {
        pub fn update_plants(&mut self)
        {
            for plant in self.plants
            {
                let hygro_avg:f32 = plant.hygrometers.iter().sum<f32>() / plant.hygrometers.len();

                // Check if the plant needs watering
                if hygro_avg <= plant.hygro_low_watter_mark
                {
                    for water_source in plant.water_sources
                    {
                        if !water_source.state
                        {
                             water_source.turn_on();
                        }
                    }
                }
                else if hygro_avg >= plant.hygro_high_watter_mark
                {
                    for water_source in plant.water_sources
                    {
                        if water_source.state
                        {
                             water_source.turn_off();
                        }
                    }
                }
            } 
        }    
    }

    // PlanWateringSystem
    struct PlantWatterSystem
    {
        plant_controller: PlantController,

    }

    impl PlantWatterSystem
    {
        
    }


}

fn main() {
    println!("Hello, world!");
}
