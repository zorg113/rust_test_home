use thiserror::Error;

#[derive(Error, Debug)]
pub enum DeviceInfoProviderError {
    #[error("Report is empty : ReportIsEmpty")]
    DeficeNotFoundInSmartHome,
}

#[derive(Error, Debug)]
pub enum SmartHouseErros {
    #[error("Problem with add_device in home : AddNotUniqueDeviceName {0}")]
    AddNotUniqueDeviceInRoom(String),
    #[error("Problem with device not found in room : DeviceNotFound {0}")]
    DeviceNotFound(String),
    #[error("Room not'found in home : NoRoomsInHouse {0}")]
    NoRoomsInHouse(String),
    #[error("Report is empty : ReportIsEmpty")]
    ReportIsEmpty,
    #[error("Device info provider : ReportIsEmpty")]
    DeviceInInfoProviderError(DeviceInfoProviderError),
    #[error("Device change content provider : RoomAlreadyExists {0}")]
    RoomAlreadyExists(String),
}

impl From<DeviceInfoProviderError> for SmartHouseErros {
    fn from(value: DeviceInfoProviderError) -> Self {
        SmartHouseErros::DeviceInInfoProviderError(value)
    }
}
