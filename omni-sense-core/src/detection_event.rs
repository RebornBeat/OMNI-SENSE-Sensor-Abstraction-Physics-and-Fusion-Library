use nalgebra::{Matrix3, Vector3};
use serde::{Deserialize, Serialize};
use crate::{FrameId, Modality, Pose, SensorId, Timestamp};

/// The contract between OMNI-SENSE and PentaTrack.
/// Every sensor pipeline produces DetectionEvent values.
/// PentaTrack and all application-layer code consume them.
/// Never changes in minor versions; changes require a major version bump.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetectionEvent {
    /// Estimated 3D position in the sensor's reported frame (meters).
    pub position: Vector3<f32>,
    /// 3×3 position covariance matrix. Never zero — minimum is sensor noise floor.
    pub position_covariance: Matrix3<f32>,
    /// Estimated velocity vector (m/s). Some if sensor provides it directly (radar Doppler).
    pub velocity: Option<Vector3<f32>>,
    /// 3×3 velocity covariance. Present iff velocity is present.
    pub velocity_covariance: Option<Matrix3<f32>>,
    /// Which modality produced this detection.
    pub modality: Modality,
    /// Which specific sensor instance produced this detection.
    pub source_sensor: SensorId,
    /// The coordinate frame this detection is expressed in.
    pub frame: FrameId,
    /// The pose of the sensor frame at the time of observation.
    pub frame_pose: Pose,
    /// Monotonic observation timestamp. Use for all ordering decisions.
    pub timestamp: Timestamp,
    /// Confidence score in [0.0, 1.0]. 1.0 = maximum sensor confidence.
    pub confidence: f32,
    /// Modality-specific classification hints. Inputs to application classifiers.
    pub hints: DetectionHints,
}

impl DetectionEvent {
    pub fn new(
        position: Vector3<f32>,
        position_covariance: Matrix3<f32>,
        modality: Modality,
        source_sensor: SensorId,
        frame: FrameId,
        timestamp: Timestamp,
        confidence: f32,
    ) -> Self {
        Self {
            position,
            position_covariance,
            velocity: None,
            velocity_covariance: None,
            modality,
            source_sensor,
            frame,
            frame_pose: Pose::default(),
            timestamp,
            confidence,
            hints: DetectionHints::default(),
        }
    }

    pub fn with_velocity(mut self, velocity: Vector3<f32>, covariance: Matrix3<f32>) -> Self {
        self.velocity = Some(velocity);
        self.velocity_covariance = Some(covariance);
        self
    }

    pub fn with_hints(mut self, hints: DetectionHints) -> Self {
        self.hints = hints;
        self
    }
}

/// Modality-specific classification hints. All fields are optional.
/// These are inputs to application-layer classifiers, not OMNI-SENSE conclusions.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DetectionHints {
    pub radar_micro_doppler: Option<RadarMicroDopplerHints>,
    pub acoustic_material: Option<AcousticMaterialHints>,
    pub rf_protocol: Option<RfProtocolHints>,
    pub eo_classification: Option<EoClassificationHints>,
    pub lidar_geometry: Option<LidarGeometryHints>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RadarMicroDopplerHints {
    /// Primary Doppler frequency in Hz (body motion).
    pub primary_doppler_hz: f32,
    /// Detected micro-Doppler sideband frequencies (limb/rotor motion).
    pub sideband_frequencies_hz: Vec<f32>,
    /// Activity-class signal strength. Higher = more confident.
    pub bipedal_gait_score: f32,
    pub rotary_signature_score: f32,
    pub vehicle_vibration_score: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AcousticMaterialHints {
    /// Material class from full-echo profiling.
    pub material_class: String,
    /// Confidence in [0.0, 1.0].
    pub material_confidence: f32,
    /// Direction of arrival in radians (azimuth, elevation).
    pub direction_of_arrival: (f32, f32),
    /// Event class (GlassBreak, Footstep, VehicleApproach, etc.)
    pub event_class: String,
    pub event_confidence: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RfProtocolHints {
    /// Detected protocol class (e.g., "ConsumerUAS_C2", "CommercialUAS_C2", "Unknown").
    pub protocol_class: String,
    pub confidence: f32,
    /// Frequency band in MHz.
    pub frequency_band_mhz: f32,
    /// Relative signal strength.
    pub rssi_dbm: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EoClassificationHints {
    /// Visual classification class.
    pub object_class: String,
    pub confidence: f32,
    /// Bounding box in image coordinates [x_min, y_min, x_max, y_max] normalized.
    pub bounding_box: Option<[f32; 4]>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LidarGeometryHints {
    /// Number of points in the cluster that produced this detection.
    pub point_count: u32,
    /// Bounding box dimensions [width, height, depth] in meters.
    pub bounding_box_m: [f32; 3],
    /// Volume of the cluster in m³.
    pub volume_m3: f32,
    /// Principal axis direction of the cluster.
    pub principal_axis: [f32; 3],
}
