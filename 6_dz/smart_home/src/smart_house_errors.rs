use thiserror::Error;


pub enum SmartErrors{
    
}

#[derive(Error, Debug)]
pub enum SmartHouseErros {
    #[error("Problem with add_device in home : AddNotUniqueDeviceName")]
    AddNotUniqueDeviceInRoom,
    #[error("Problem with device not foind in room : DeviceNotFound")]
    DeviceNotFound,
    #[error("Room not'found in home : NoRoomsInHouse")]
    NoRoomsInHouse,
    #[error("Report is empty : ReportIsEmpty")]
    ReportIsEmpty,
}

#[derive(Error, Debug)]
pub enum DeviceInfoProviderError{

}