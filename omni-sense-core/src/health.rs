use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SensorHealth {
    Online,
    Degraded { quality: u8 }, // 0–100 quality percentage
    Offline,
    KillSwitchEngaged,        // Identity nodes only
    Initializing,
}
