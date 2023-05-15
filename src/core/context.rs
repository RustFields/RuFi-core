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
    /// * `selfId` The ID of the device that this context is for.
    ///
    /// * `local_sensor` The values perceived by the local sensors of the device.
    ///
    /// * `nbr_sensor` The values perceived by the sensors for each neighbor of the device.
    ///
    /// * `exports` All the export that are available to the device.
    #[derive(Debug)]
    pub struct Context {
        pub(crate) self_id: i32,
        pub(crate) local_sensor: HashMap<SensorId, Box<dyn Any>>,
        pub(crate) nbr_sensor: HashMap<SensorId, HashMap<i32, Box<dyn Any>>>,
        pub(crate) exports: HashMap<i32, Export>
    }
}

impl Context {
    /// ## Create new Context of a device from the given parameters.
    ///
    /// ### Arguments
    ///
    /// * `self_id` - the ID of the device
    ///
    /// * `local_sensor` - The values perceived by the local sensors of the device.
    ///
    /// * `nbr_sensor` - The values perceived by the sensors for each neighbor of the device.
    ///
    /// * `exports` - All the export that are available to the device.
    pub fn new(
        self_id: i32,
        local_sensor: HashMap<SensorId, Box<dyn Any>>,
        nbr_sensor: HashMap<SensorId, HashMap<i32, Box<dyn Any>>>,
        exports: HashMap<i32, Export>) -> Context {
        Context {
            self_id,
            local_sensor,
            nbr_sensor,
            exports
        }
    }

    /// Add an export of a device to the context.
    ///
    /// * `id`  the ID of the device
    /// * `data` the export of the device
    pub fn put_export(&mut self, id: i32, data: Export) {
        self.exports.insert(id, data);
    }

    /// Read the value corresponding to the given path from the export of a device.
    ///
    /// * `id` the ID of the device
    /// * `path` the path to the value
    /// * `A` the type of the value
    /// * return the value if it exists
    pub fn read_export_value<A: 'static>(&self, id: i32, path: Path) -> Option<&A> {
        self.exports.get(&id).and_then(|export| export.get(&path))
    }

    /// Get the value of the given sensor.
    /// * `name` the name of the sensor
    /// * `T` the type of the value
    /// * return the value if it exists
    pub fn local_sense<A: 'static>(&self, local_sensor_id: SensorId) -> Option<&A> {
        self.local_sensor.get(&local_sensor_id).and_then(|value| value.downcast_ref::<A>())
    }

    /// Get the value of the given sensor for the given neighbor.
    /// * `sensor_id` the neighbor sensor id
    /// * `nbr_id` the neighbor id
    /// * `T` the type of the value
    /// * return the value if it exists
    pub fn nbr_sense<A: 'static>(&self, sensor_id: SensorId, nbr_id: i32) -> Option<A> {
        // self.nbr_sensor.get(&sensor_id).map(|value| value.get(&nbr_id)).and_then(|value| value.downcast_ref::<A>())
        unimplemented!("todo")
    }
}

#[cfg(test)]
mod test {
    use std::any::Any;
    use std::collections::HashMap;
    use crate::core::context::context::Context;
    use crate::core::export::export::Export;
    use crate::core::path::path::path::Path;
    use crate::core::path::slot::slot::Slot::{Branch, Nbr, Rep};
    use crate::core::sensor_id::sensor_id::SensorId;

    #[test]
    fn test_new() {
        let local_sensor: HashMap<SensorId, Box<dyn Any>> = HashMap::from([(SensorId::new("test".to_string()), Box::new(10) as Box<dyn Any>)]);
        let nbr_sensor: HashMap<SensorId, HashMap<i32, Box<dyn Any>>> = HashMap::from([(SensorId::new("test".to_string()), HashMap::from([(0, Box::new(10) as Box<dyn Any>)]))]);
        let current_export: HashMap<i32, Export> = HashMap::from([(0, Export::new(HashMap::from([(Path::new(vec![Rep(0), Nbr(0)]), Box::new(10) as Box<dyn Any>)])))]);
        let context = Context::new(7, local_sensor, nbr_sensor, current_export);
        assert_eq!(context.self_id, 7);
    }

    #[test]
    fn test_put_export(){
        let local_sensor: HashMap<SensorId, Box<dyn Any>> = HashMap::from([(SensorId::new("test".to_string()), Box::new(10) as Box<dyn Any>)]);
        let nbr_sensor: HashMap<SensorId, HashMap<i32, Box<dyn Any>>> = HashMap::from([(SensorId::new("test".to_string()), HashMap::from([(0, Box::new(10) as Box<dyn Any>)]))]);
        let current_export: HashMap<i32, Export> = HashMap::from([(0, Export::new(HashMap::from([(Path::new(vec![Rep(0), Nbr(0)]), Box::new(10) as Box<dyn Any>)])))]);
        let mut context = Context::new(7, local_sensor, nbr_sensor, current_export);
        assert_eq!(context.exports.len(), 1);
        let add_export = Export::new(HashMap::from([(Path::new(vec![Branch(0), Nbr(0)]), Box::new(5) as Box<dyn Any>)]));
        context.put_export(1, add_export);
        assert_eq!(context.exports.len(), 2)
    }

    #[test]
    fn test_read_export_value() {
        let local_sensor: HashMap<SensorId, Box<dyn Any>> = HashMap::from([(SensorId::new("test".to_string()), Box::new(10) as Box<dyn Any>)]);
        let nbr_sensor: HashMap<SensorId, HashMap<i32, Box<dyn Any>>> = HashMap::from([(SensorId::new("test".to_string()), HashMap::from([(0, Box::new(10) as Box<dyn Any>)]))]);
        let current_export: HashMap<i32, Export> = HashMap::from([(0, Export::new(HashMap::from([(Path::new(vec![Rep(0), Nbr(0)]), Box::new(10) as Box<dyn Any>)])))]);
        let context = Context::new(7, local_sensor, nbr_sensor, current_export);
        assert_eq!(context.read_export_value::<i32>(0, Path::new(vec![Rep(0), Nbr(0)])).unwrap(), &10);
    }

    #[test]
    fn test_local_sense(){
        let local_sensor: HashMap<SensorId, Box<dyn Any>> = HashMap::from([(SensorId::new("test".to_string()), Box::new(10) as Box<dyn Any>)]);
        let nbr_sensor: HashMap<SensorId, HashMap<i32, Box<dyn Any>>> = HashMap::from([(SensorId::new("test".to_string()), HashMap::from([(0, Box::new(10) as Box<dyn Any>)]))]);
        let current_export: HashMap<i32, Export> = HashMap::from([(0, Export::new(HashMap::from([(Path::new(vec![Rep(0), Nbr(0)]), Box::new(10) as Box<dyn Any>)])))]);
        let context = Context::new(7, local_sensor, nbr_sensor, current_export);
        assert_eq!(context.local_sense::<i32>(SensorId::new("test".to_string())).unwrap(), &10);
    }
}