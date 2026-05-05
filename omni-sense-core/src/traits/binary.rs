use serde::{Deserialize, Serialize};
use crate::{SensorResult, Timestamp};
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BinaryEvent {
    pub active: bool,
    pub timestamp: Timestamp,
}

pub trait BinarySensor: crate::traits::sensor::Sensor {
    fn poll_state(&mut self) -> SensorResult<BinaryEvent>;
    fn debounce_duration(&self) -> Duration;
}
