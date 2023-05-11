use std::any::Any;
use std::collections::HashMap;
use crate::core::context::context::Context;
use crate::core::export::export::Export;
use crate::core::path::path::path::Path;
use crate::core::sensor_id::sensor_id::SensorId;

pub mod context {
    use std::any::Any;
    use std::collections::HashMap;
    use crate::core::export::export::Export;
    use crate::core::sensor_id::sensor_id::SensorId;

    /// Context implementation
    ///
    /// * `selfId` the ID of the device
    ///
    /// * `currentExports` the exports of the neighbours
    ///
    /// * `localSensor` TODO (in hello-scafi whether the sensor is the source)
    ///
    /// * `nbrSensor` map each neighbours to the corresponding value
    #[derive(Debug)]
    pub struct Context {
        pub(crate) self_id: i32,
        pub(crate) local_sensor: HashMap<SensorId, Box<dyn Any>>,
        pub(crate) nbr_sensor: HashMap<SensorId, HashMap<i32, Box<dyn Any>>>,
        pub(crate) current_exports: Vec<(i32, Export)>
    }
}

impl Context {
    /// ## Create new Context from the given parameters.
    ///
    /// ### Arguments
    ///
    /// * `self_id` - the ID of the device
    ///
    /// * `local_sensor` - TODO
    ///
    /// * `nbr_sensor` - map each neighbours to the corresponding value
    ///
    /// * `current_export` - the exports of the neighbours
    pub fn new(
        self_id: i32,
        local_s: HashMap<SensorId, Box<dyn Any>>,
        nbr_s: HashMap<SensorId, HashMap<i32, Box<dyn Any>>>,
        current_ex: Vec<(i32, Export)>) -> Context {
        Context {
            self_id,
            local_sensor: local_s,
            nbr_sensor: nbr_s,
            current_exports: current_ex.into_iter().collect()
        }
    }

    pub fn exports_map(&self) -> HashMap<i32, Export> {
        unimplemented!("TODO implement the exports map")
    }

    pub fn update_export(&self, id: i32, data: Export) {
        unimplemented!("TODO implement the update export")
    }

    pub fn exports(&self) -> Vec<(i32, Export)> {
        unimplemented!("TODO implement the exports")
    }

    pub fn read_slot<A>(&self, index: i32, path: Path) -> Option<&A> {
        // self.exports_map().get(&index).and_then(|export| export.get(&path))
        unimplemented!("TODO implement the read slot")
    }

    pub fn to_string(&self) -> String {
        unimplemented!("TODO implement the to string")
    }

    pub fn sense<A>(&self, local_sensor_id: SensorId) -> Option<A> {
        unimplemented!("TODO implement sense")
    }

    pub fn nbr_sense<A>(&self, nbr_sensor_id: SensorId, nbr_id: i32) -> Option<A> {
        unimplemented!("TODO implement nbr sense")
    }
}

#[cfg(test)]
mod test {
    use std::any::Any;
    use std::collections::HashMap;
    use crate::core::context::context::Context;
    use crate::core::export::export::Export;
    use crate::core::path::path::path::Path;
    use crate::core::path::slot::slot::Slot::{Nbr, Rep};
    use crate::core::sensor_id::sensor_id::SensorId;

    #[test]
    fn test_exports_map() {
        unimplemented!("TODO test the exports map")
    }

    #[test]
    fn test_new() {
        let local_sensor: HashMap<SensorId, Box<dyn Any>> = HashMap::from([(SensorId::new("test".to_string()), Box::new(10) as Box<dyn Any>)]);
        let nbr_sensor: HashMap<SensorId, HashMap<i32, Box<dyn Any>>> = HashMap::from([(SensorId::new("test".to_string()), HashMap::from([(0, Box::new(10) as Box<dyn Any>)]))]);
        let current_export: Vec<(i32, Export)> = Vec::from([(0, Export::new(HashMap::from([(Path::new(vec![Rep(0), Nbr(0)]), Box::new(10) as Box<dyn Any>)])))]);
        let context = Context::new(7, local_sensor, nbr_sensor, current_export);
        assert_eq!(context.self_id, 7);
    }

    #[test]
    fn test_read_slot() {
        // let mut map: HashMap<Path, Box<dyn Any>> = HashMap::new();
        // map.insert(Path::new(vec![Rep(0), Nbr(0)]), Box::new(10));
        // let export = Export::new(map);
        unimplemented!("TODO")
    }
}