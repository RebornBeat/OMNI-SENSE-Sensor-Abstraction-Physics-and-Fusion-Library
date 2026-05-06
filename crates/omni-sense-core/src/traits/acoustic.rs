use serde::{Deserialize, Serialize};
use nalgebra::Vector3;
use crate::{SensorResult, Timestamp};

/// Geometry of a microphone array.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArrayGeometry {
    /// Position of each microphone in sensor frame (meters).
    pub microphone_positions: Vec<Vector3<f32>>,
}

/// One frame of multi-channel audio data.
#[derive(Debug, Clone)]
pub struct AcousticFrame {
    /// channels × samples, row-major. channels = microphone_positions.len().
    pub samples: Vec<Vec<f32>>,
    pub sample_rate_hz: u32,
    pub timestamp: Timestamp,
}

pub trait AcousticSensor: crate::traits::sensor::Sensor {
    fn poll_frame(&mut self) -> SensorResult<AcousticFrame>;
    fn array_geometry(&self) -> &ArrayGeometry;
    fn sample_rate_hz(&self) -> u32;
    fn channel_count(&self) -> u32;
}
