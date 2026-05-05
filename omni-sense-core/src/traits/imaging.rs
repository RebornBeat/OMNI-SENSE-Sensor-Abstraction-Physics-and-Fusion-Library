//! Conventional camera trait.
//! Only available with the `camera` feature flag.
//! Projects that do not depend on `omni-sense-drivers-camera` have camera-free
//! sensing by construction — this type does not exist in their compilation.

use crate::{SensorResult, Timestamp};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageFrame {
    /// Raw pixel data. Format depends on sensor configuration.
    pub data: Vec<u8>,
    pub width: u32,
    pub height: u32,
    pub channels: u8,
    pub timestamp: Timestamp,
}

pub trait ImagingSensor: crate::traits::sensor::Sensor {
    fn poll_frame(&mut self) -> SensorResult<ImageFrame>;
    fn resolution(&self) -> (u32, u32);
    fn frame_rate_hz(&self) -> f32;
}
