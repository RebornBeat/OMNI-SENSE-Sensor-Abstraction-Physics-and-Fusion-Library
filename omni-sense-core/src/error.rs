use thiserror::Error;

#[derive(Debug, Error)]
pub enum SensorError {
    #[error("Sensor not initialized")]
    NotInitialized,
    #[error("Hardware communication error: {0}")]
    HardwareError(String),
    #[error("Sensor offline")]
    Offline,
    #[error("Kill switch engaged — sensor disabled by hardware")]
    KillSwitchEngaged,
    #[error("Configuration error: {0}")]
    ConfigurationError(String),
    #[error("Timeout waiting for sensor response")]
    Timeout,
    #[error("Data parsing error: {0}")]
    ParseError(String),
}

pub type SensorResult<T> = Result<T, SensorError>;
