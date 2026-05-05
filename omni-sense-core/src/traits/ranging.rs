use nalgebra::Vector3;
use serde::{Deserialize, Serialize};
use crate::{SensorResult, Timestamp};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RangeReturn {
    /// Range from sensor to detection in meters.
    pub range_m: f32,
    /// Bearing in radians (azimuth from sensor X-axis, positive toward Y).
    pub bearing_rad: f32,
    /// Elevation in radians (positive upward from horizontal).
    pub elevation_rad: f32,
    /// Reflected intensity (sensor-specific units; normalized [0,1] recommended).
    pub intensity: f32,
    /// Timestamp of this specific return (not the poll call).
    pub timestamp: Timestamp,
}

impl RangeReturn {
    /// Convert to Cartesian coordinates in sensor frame.
    pub fn to_cartesian(&self) -> Vector3<f32> {
        let cos_el = self.elevation_rad.cos();
        Vector3::new(
            self.range_m * cos_el * self.bearing_rad.cos(),
            self.range_m * cos_el * self.bearing_rad.sin(),
            self.range_m * self.elevation_rad.sin(),
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BeamPattern {
    /// Horizontal half-angle in radians.
    pub azimuth_half_angle_rad: f32,
    /// Vertical half-angle in radians.
    pub elevation_half_angle_rad: f32,
    /// Nominal beam divergence half-angle in radians.
    pub beam_divergence_half_angle_rad: f32,
}

pub trait RangingSensor: crate::traits::sensor::Sensor {
    fn poll_ranges(&mut self) -> SensorResult<Vec<RangeReturn>>;
    fn nominal_range_m(&self) -> f32;
    fn beam_pattern(&self) -> BeamPattern;
    fn range_resolution_m(&self) -> f32;
}
