// Пользовательские устройства:
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
    fn get_device_info(&self, room: &str, name: &str) -> String;
}

pub trait DeviceLocationProvider {
    fn get_device_location_and_name(&self) -> Vec<(String, String)>;
}
