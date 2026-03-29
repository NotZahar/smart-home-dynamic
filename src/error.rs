use thiserror::Error;

#[derive(Debug, Error)]
pub enum HomeError {
    #[error("Room not found: {0}")]
    RoomNotFound(String),
    #[error("Device not found: {0}")]
    DeviceNotFound(String),
}
