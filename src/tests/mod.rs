
#[cfg(test)]
mod tests
{
    use crate::plant_care_system;
    use crate::plant_care_system::device::Device;
    use plant_care_system as pcs;
    use pcs::water_source as pws;
    use pcs::device::State;
    use pcs::device;

    #[test]
    fn water_source_test()
    {
        let p : pws::WaterSource = pws::WaterSource::new();

        let s : State = p.state();
        
        assert_eq!(s, State::Off);
    }
}
