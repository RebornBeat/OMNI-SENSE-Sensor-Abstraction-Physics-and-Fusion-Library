use serde::{Deserialize, Serialize};

/// The sensing modality that produced a detection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Modality {
    MmWaveRadar,
    ScanningLidar,
    SolidStateLidar,
    FlashLidar,
    EventCamera,
    MicrophoneArray,
    Imu,
    Pir,
    ToF1D,
    Environmental,
    Camera,
    RfPassive,
    Fused,
}
