use thiserror::Error;

#[derive(Error, Debug)]
pub enum DeviceInfoProviderError{
    #[error("Report is empty : ReportIsEmpty")]
    DeficeNotFoundInSmartHome,
}

#[derive(Error, Debug)]
pub enum SmartHouseErros {
    #[error("Problem with add_device in home : AddNotUniqueDeviceName")]
    AddNotUniqueDeviceInRoom,
    #[error("Problem with device not found in room : DeviceNotFound")]
    DeviceNotFound,
    #[error("Room not'found in home : NoRoomsInHouse")]
    NoRoomsInHouse,
    #[error("Report is empty : ReportIsEmpty")]
    ReportIsEmpty,
    #[error("Device info provider : ReportIsEmpty")]
    DeviceInInfoProviderError(DeviceInfoProviderError),

}

impl From<DeviceInfoProviderError> for SmartHouseErros{
    fn from(value: DeviceInfoProviderError) -> Self {
        SmartHouseErros::DeviceInInfoProviderError(value)
    }
}

