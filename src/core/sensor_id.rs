use crate::core::sensor_id::sensor_id::SensorId;

///Sensor is piece of hardware that allow the device to interact with the environment.
pub mod sensor_id {
    pub struct SensorId {
        pub(crate) sensor_id: String
    }
}

impl SensorId {
    /// Given a string, creates a new sensor id
    pub fn new(id: String) -> SensorId {
        SensorId {
            sensor_id: id
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::core::sensor_id::sensor_id::SensorId;

    #[test]
    fn test_new() {
        let sensor_id = SensorId::new("test".to_string());
        assert_eq!(sensor_id.sensor_id, "test".to_string())
    }
}