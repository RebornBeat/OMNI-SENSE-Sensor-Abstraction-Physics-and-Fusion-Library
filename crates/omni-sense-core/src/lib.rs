//! OMNI-SENSE Core — sensor traits, DetectionEvent, and shared types.
//!
//! This crate is the foundation of the OMNI-SENSE ecosystem. Every other
//! crate depends on it. Every sensor driver implements its traits.
//! PentaTrack consumes its DetectionEvent type.

pub mod detection_event;
pub mod error;
pub mod frame;
pub mod health;
pub mod modality;
pub mod pose;
pub mod sensor_id;
pub mod timestamp;
pub mod traits;

pub use detection_event::{DetectionEvent, DetectionHints, AcousticMaterialHints,
    EoClassificationHints, LidarGeometryHints, RadarMicroDopplerHints, RfProtocolHints};
pub use error::{SensorError, SensorResult};
pub use frame::FrameId;
pub use health::SensorHealth;
pub use modality::Modality;
pub use pose::Pose;
pub use sensor_id::SensorId;
pub use timestamp::Timestamp;
pub use traits::{
    sensor::Sensor,
    ranging::{RangingSensor, RangeReturn, BeamPattern},
    velocity::{VelocitySensor, VelocityReturn},
    event_sensor::{EventSensor, PixelEvent},
    acoustic::{AcousticSensor, AcousticFrame, ArrayGeometry},
    imu::{ImuSensor, ImuSample, EnvironmentQuantity},
    binary::{BinarySensor, BinaryEvent},
    environmental::{EnvironmentalSensor, EnvironmentSample},
};

#[cfg(feature = "camera")]
pub use traits::imaging::{ImagingSensor, ImageFrame};
