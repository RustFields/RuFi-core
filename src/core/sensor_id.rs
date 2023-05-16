use crate::core::sensor_id::sensor_id::SensorId;

///Sensor is piece of hardware that allow the device to interact with the environment.
pub mod sensor_id {
    #[derive(PartialEq, Debug, Clone, Eq, Hash)]
    pub struct SensorId {
        pub(crate) name: String,
    }
}

impl SensorId {
    /// Given a string, creates a new sensor id
    pub fn new(name: String) -> SensorId {
        SensorId { name }
    }
}

#[cfg(test)]
mod tests {
    use crate::core::sensor_id::sensor_id::SensorId;

    #[test]
    fn test_new() {
        let sensor_id = SensorId::new("foo".to_string());
        assert_eq!(sensor_id.name, "foo".to_string())
    }
}
