use nalgebra::{UnitQuaternion, Vector3};
use serde::{Deserialize, Serialize};

/// 6-DOF pose: position + orientation.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Pose {
    pub position: Vector3<f32>,
    pub orientation: UnitQuaternion<f32>,
}

impl Default for Pose {
    fn default() -> Self {
        Self {
            position: Vector3::zeros(),
            orientation: UnitQuaternion::identity(),
        }
    }
}

impl Pose {
    pub fn new(position: Vector3<f32>, orientation: UnitQuaternion<f32>) -> Self {
        Self { position, orientation }
    }

    pub fn at_position(position: Vector3<f32>) -> Self {
        Self { position, orientation: UnitQuaternion::identity() }
    }
}
