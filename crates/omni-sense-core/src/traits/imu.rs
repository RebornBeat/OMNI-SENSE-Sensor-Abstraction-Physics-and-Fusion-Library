use nalgebra::Vector3;
use serde::{Deserialize, Serialize};
use crate::{SensorResult, Timestamp};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImuSample {
    /// Acceleration including gravity, m/s² in sensor frame.
    pub accel_ms2: Vector3<f32>,
    /// Angular velocity, rad/s in sensor frame.
    pub gyro_rads: Vector3<f32>,
    /// Magnetic field, μT in sensor frame (None if no magnetometer).
    pub mag_ut: Option<Vector3<f32>>,
    pub timestamp: Timestamp,
}

pub trait ImuSensor: crate::traits::sensor::Sensor {
    fn poll_imu(&mut self) -> SensorResult<ImuSample>;
    fn accelerometer_range_g(&self) -> f32;
    fn gyroscope_range_dps(&self) -> f32;
    fn has_magnetometer(&self) -> bool;
    fn sample_rate_hz(&self) -> f32;
}

/// Environmental quantity identifier for EnvironmentalSensor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EnvironmentQuantity {
    TemperatureCelsius,
    HumidityPercent,
    PressureHpa,
    Co2Ppm,
    VocIndex,
    Pm25UgM3,
    Pm10UgM3,
}
