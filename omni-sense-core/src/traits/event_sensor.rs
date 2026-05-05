use serde::{Deserialize, Serialize};
use crate::{SensorResult, Timestamp};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PixelEvent {
    pub x: u16,
    pub y: u16,
    /// true = brightness increased, false = brightness decreased.
    pub polarity: bool,
    pub timestamp: Timestamp,
}

pub trait EventSensor: crate::traits::sensor::Sensor {
    fn poll_events(&mut self) -> SensorResult<Vec<PixelEvent>>;
    fn resolution(&self) -> (u32, u32);
    fn contrast_threshold(&self) -> f32;
    fn dynamic_range_db(&self) -> f32;
}
