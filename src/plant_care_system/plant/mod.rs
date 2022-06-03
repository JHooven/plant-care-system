
use super::hygrometer::Hygrometer;
use super::water_source::WaterSource;

mod tests;

pub struct Plant
{
    pub name: String,
    pub hygrometers: Vec<Hygrometer>,
    pub water_sources: Vec<WaterSource>,
    pub hygro_low_watter_mark: f32,
    pub hygro_high_watter_mark: f32,
}

impl Plant
{
    // Name of the plant
    pub fn name(&self) -> &String {&self.name}
    pub fn hygrometers(&self) -> &Vec<Hygrometer> {&self.hygrometers}
    pub fn water_sources(&self) -> &Vec<WaterSource> {&self.water_sources}
    pub fn hygro_avg(&mut self) -> f32 
    {
        let mut total: f32 = 0.0;

        for hygro in self.hygrometers.iter_mut()
        {
            total += hygro.update_value();
        } 

        total / (self.hygrometers.len() as f32)
    }

    pub fn new
    (
          name: String
    ) -> Plant
    {
        let hygrometers = Vec::<Hygrometer>::new();
        
        let water_sources = Vec::<WaterSource>::new();

        Plant
        { 
              name
            , hygrometers
            , water_sources
            , hygro_low_watter_mark: 0.0
            , hygro_high_watter_mark: 400.0
       }
    }
}

impl PartialEq for Plant 
{
    fn eq(&self, other: &Plant) -> bool 
    {  
           self.name == other.name
    }
}
