use serde::{Deserialize, Serialize};
use crate::{SensorResult, Timestamp};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VelocityReturn {
    /// Range in meters.
    pub range_m: f32,
    /// Bearing in radians.
    pub bearing_rad: f32,
    /// Elevation in radians.
    pub elevation_rad: f32,
    /// Radial velocity in m/s (positive = approaching).
    pub radial_velocity_ms: f32,
    /// Confidence in [0, 1].
    pub confidence: f32,
    pub timestamp: Timestamp,
}

pub trait VelocitySensor: crate::traits::sensor::Sensor {
    fn poll_velocities(&mut self) -> SensorResult<Vec<VelocityReturn>>;
    fn velocity_resolution_ms(&self) -> f32;
    fn unambiguous_velocity_range_ms(&self) -> (f32, f32);
}
