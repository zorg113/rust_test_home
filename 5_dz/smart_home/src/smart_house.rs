
use crate::smart_devices::DeviceInfoProvider;
use crate::smart_devices::DeviceLocationProvider;
use std::collections::HashMap;
use std::collections::HashSet;
struct Room {
    devices: HashSet<String>,
}

impl Room {
    fn new() -> Self {
        let dev_: HashSet<String> = HashSet::new();
        Room { devices: dev_ }
    }
}

pub struct SmartHouse {
    name: String,
    rooms: HashMap<String, Room>,
}

impl SmartHouse {
    pub fn new(name_house: String) -> Self {
        SmartHouse {
            name: name_house,
            rooms: HashMap::new(),
        }
    }

    pub fn get_rooms(&self) -> Vec<&String> {
        let mut out: Vec<&String> = Vec::new();
        for name in self.rooms.keys() {
            out.push(name);
        }
        out
    }

    pub fn get_name(self) -> String {
        self.name
    }

    pub fn devices(&self, room: &str) -> Vec<&String> {
        let mut out: Vec<&String> = Vec::new();
        for el in &self.rooms[room].devices {
            out.push(el);
        }
        out
    }

    pub fn add_devices(&mut self, dev_provider: &dyn DeviceLocationProvider) -> bool {
        let location_devices = dev_provider.get_device_location_and_name();
        for val in location_devices {
            if let std::collections::hash_map::Entry::Vacant(e) = self.rooms.entry(val.0.clone()) {
                let mut room: Room = Room::new();
                room.devices.insert(val.1);
                e.insert(room);
            } else if self.rooms[&val.0].devices.contains(&val.1) {
                return false;
            } else {
                self.rooms.get_mut(&val.0).unwrap().devices.insert(val.1);
            }
        }
        true
    }

    pub fn create_report(&self, dev_provider: &dyn DeviceInfoProvider) -> String {
        let mut info: String = "".to_string();
        for (name_room, room) in &self.rooms {
            for dev in &room.devices {
                let dat: String = dev_provider.get_device_info(name_room, dev);
                if !dat.is_empty() {
                    info = format!("{} {} {}\n\t\t   ", info, name_room, dat);
                }
            }
        }
        if info.is_empty() {
            info = "Devices not found".to_string();
        }
        info
    }

    pub fn home_report(self) -> String {
        let mut out: String = "".to_string();
        out = format!("{} SMART HOME: {}\n", out, self.name);
        for (name_room, room) in self.rooms {
            out = format!("{} Room Name: {}\n\t", out, name_room);
            for device in room.devices {
                out = format!("{} Device: {}\n\t", out, device);
            }
            out = format!("{}\n", out);
        }
        out
    }
}
