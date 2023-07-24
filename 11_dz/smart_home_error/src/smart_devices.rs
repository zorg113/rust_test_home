// Пользовательские устройства:
use crate::smart_house_errors::*;
use std::fmt;
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

impl fmt::Display for SmartSocket {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "status: {},
             name: {},
             room: {},",
            self.status, self.name, self.room
        )
    }
}

impl fmt::Display for SmartThermometer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "status: {},
             name: {},
             room: {},",
            self.status, self.name, self.room
        )
    }
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
