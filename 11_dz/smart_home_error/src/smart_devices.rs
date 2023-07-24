// Пользовательские устройства:
use crate::smart_house_errors::*;
pub struct SmartSocket {
    pub status: String,
    pub name: String,
    pub room: String,
}
pub struct SmartThermometer {
    pub status: String,
    pub name: String,
    pub room: String,
}

pub trait DeviceInfoProvider {
    fn get_device_info(&self, room: &str, name: &str) -> Result<String, DeviceInfoProviderError>;
}

pub trait DeviceChangeContentProvider {
    fn add_device_into_room(&self) -> (&str, &str);

    fn add_room_in_home(&self) -> &str;

    fn delete_device_in_room(&self) -> (&str, &str);

    fn delete_room(&self) -> &str;
}

pub trait DeviceLocationProvider {
    fn get_device_location_and_name(&self) -> Vec<(String, String)>;
}
