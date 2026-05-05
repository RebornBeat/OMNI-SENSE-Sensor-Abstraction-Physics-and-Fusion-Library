use crate::{FrameId, Pose, SensorHealth, SensorId, SensorResult};
use crate::modality::Modality;

/// Base trait for all OMNI-SENSE sensors.
/// Timestamp discipline: every polled observation must record the timestamp
/// at the earliest possible point — at interrupt entry or immediately after
/// hardware read, before any processing.
pub trait Sensor: Send {
    fn id(&self) -> SensorId;
    fn modality(&self) -> Modality;
    fn frame(&self) -> FrameId;
    fn pose(&self) -> Pose;
    fn health(&self) -> SensorHealth;
    fn initialize(&mut self) -> SensorResult<()>;
    fn shutdown(&mut self) -> SensorResult<()>;
}
