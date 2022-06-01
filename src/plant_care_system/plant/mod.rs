
use super::hygrometer::Hygrometer;
use super::water_source::WaterSource;
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

        for hygro in self.hygrometers.iter()
        {
            total += hygro.last_value();
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