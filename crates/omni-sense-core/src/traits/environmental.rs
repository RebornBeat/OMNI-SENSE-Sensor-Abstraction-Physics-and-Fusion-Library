use serde::{Deserialize, Serialize};
use crate::{SensorResult, Timestamp};
use crate::traits::imu::EnvironmentQuantity;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvironmentSample {
    pub values: Vec<(EnvironmentQuantity, f32)>,
    pub timestamp: Timestamp,
}

pub trait EnvironmentalSensor: crate::traits::sensor::Sensor {
    fn poll_environment(&mut self) -> SensorResult<EnvironmentSample>;
    fn measured_quantities(&self) -> &[EnvironmentQuantity];
}
