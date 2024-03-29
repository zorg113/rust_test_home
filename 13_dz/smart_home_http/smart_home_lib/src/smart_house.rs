use crate::smart_devices::*;
use crate::smart_house_errors::*;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Clone, Serialize, Deserialize)]
pub struct Room {
    devices: HashSet<String>,
}

impl Room {
    fn new() -> Self {
        let dev_: HashSet<String> = HashSet::new();
        Room { devices: dev_ }
    }
}
#[derive(Clone, Serialize, Deserialize)]
pub struct SmartHouse {
    name: String,
    rooms: HashMap<String, Room>,
}

// for report
#[derive(Clone, Serialize, Deserialize)]
struct ReportDevice {
    name: String,
    status: String,
}
#[derive(Clone, Serialize, Deserialize)]
struct ReportRoom {
    room_name: String,
    devices: Vec<ReportDevice>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Report {
    rooms: Vec<ReportRoom>,
}

pub trait Formatter {
    fn format(&self, key: &str, data: &SmartHouse, buf: &mut String);
}

impl SmartHouse {
    pub fn new(name_house: String) -> Self {
        SmartHouse {
            name: name_house,
            rooms: HashMap::new(),
        }
    }

    pub fn add_room(
        &mut self,
        dev_provider: &dyn RoomChangeContentProvider,
    ) -> Result<bool, SmartHouseErros> {
        let room = dev_provider.add_room_in_home();
        if self.rooms.contains_key(room) {
            return Err(SmartHouseErros::RoomAlreadyExists(room.to_string()));
        }
        self.rooms.insert(room.to_string(), Room::new());
        Ok(true)
    }

    pub fn add_device(
        &mut self,
        dev_provider: &dyn DeviceChangeContentProvider,
    ) -> Result<bool, SmartHouseErros> {
        let (room, device) = dev_provider.add_device_into_room();
        if self.rooms.contains_key(&room.to_string()) {
            // проверку на наличие уникальных имен
            if self.rooms[room].devices.contains(device) {
                return Err(SmartHouseErros::AddNotUniqueDeviceInRoom(
                    device.to_string(),
                ));
            }
            self.rooms
                .get_mut(room)
                .unwrap()
                .devices
                .insert(device.to_string());
            return Ok(true);
        }
        let mut room_ = Room::new();
        room_.devices.insert(device.to_string());
        self.rooms.insert(room.to_string(), room_);
        Ok(true)
    }

    pub fn delete_room(
        &mut self,
        dev_provider: &dyn RoomChangeContentProvider,
    ) -> Result<bool, SmartHouseErros> {
        let room = dev_provider.delete_room().to_string();
        if self.rooms.contains_key(&room) {
            self.rooms.remove(&room);
            return Ok(true);
        }
        Err(SmartHouseErros::NoRoomsInHouse(room.to_string()))
    }

    pub fn delete_device(
        &mut self,
        dev_provider: &dyn DeviceChangeContentProvider,
    ) -> Result<bool, SmartHouseErros> {
        let (room, device) = dev_provider.delete_device_in_room();
        if self.rooms.contains_key(room) {
            if self.rooms[room].devices.contains(device) {
                self.rooms.get_mut(room).unwrap().devices.remove(device);
                return Ok(true);
            }
            return Err(SmartHouseErros::DeviceNotFound(device.to_string()));
        }
        Err(SmartHouseErros::NoRoomsInHouse(room.to_string()))
    }

    pub fn get_rooms(&self) -> Result<Vec<&String>, SmartHouseErros> {
        let mut out: Vec<&String> = Vec::new();
        for name in self.rooms.keys() {
            out.push(name);
        }
        if out.is_empty() {
            return Err(SmartHouseErros::NoRoomsInHouse("IS EMPTY".to_string()));
        }
        Ok(out)
    }

    pub fn get_name(self) -> String {
        self.name
    }

    pub fn devices(&self, room: &str) -> Option<Vec<&String>> {
        let mut out: Vec<&String> = Vec::new();
        for el in &self.rooms[room].devices {
            out.push(el);
        }
        if out.is_empty() {
            return None;
        }
        Some(out)
    }

    pub fn add_devices(
        &mut self,
        dev_provider: &dyn DeviceLocationProvider,
    ) -> Result<bool, SmartHouseErros> {
        let location_devices = dev_provider.get_device_location_and_name();
        for val in location_devices {
            if let std::collections::hash_map::Entry::Vacant(e) = self.rooms.entry(val.0.clone()) {
                let mut room: Room = Room::new();
                room.devices.insert(val.1);
                e.insert(room);
            } else if self.rooms[&val.0].devices.contains(&val.1) {
                return Err(SmartHouseErros::AddNotUniqueDeviceInRoom(val.1));
            } else {
                self.rooms.get_mut(&val.0).unwrap().devices.insert(val.1);
            }
        }
        Ok(true)
    }

    pub fn create_report(
        &self,
        dev_provider: &dyn DeviceInfoProvider,
    ) -> Result<String, SmartHouseErros> {
        let mut info: String = "".to_string();
        for (name_room, room) in &self.rooms {
            for dev in &room.devices {
                let dat: String = dev_provider.get_device_info(name_room, dev)?;
                if !dat.is_empty() {
                    info = format!("{} {} {}\n\t\t   ", info, name_room, dat);
                }
            }
        }
        if info.is_empty() {
            return Err(SmartHouseErros::DeviceNotFound("EMPTY".to_string()));
        }
        Ok(info)
    }

    pub fn create_report_new(
        &self,
        dev_provider: &dyn DeviceInfoProvider,
    ) -> Result<Report, SmartHouseErros> {
        let mut report = Report { rooms: vec![] };
        for (name_room, room) in &self.rooms {
            let mut rep_room: ReportRoom = ReportRoom {
                room_name: "".to_string(),
                devices: vec![],
            };
            for dev in &room.devices {
                let dat: String = dev_provider.get_device_info(name_room, dev)?;
                if !dat.is_empty() {
                    rep_room.room_name = name_room.clone();
                    rep_room.devices.push(ReportDevice {
                        name: dev.clone(),
                        status: dat,
                    })
                }
            }
            report.rooms.push(rep_room);
        }
        if report.rooms.is_empty() {
            return Err(SmartHouseErros::DeviceNotFound("EMPTY".to_string()));
        }
        Ok(report)
    }

    pub fn home_report(self) -> Result<String, SmartHouseErros> {
        let mut out: String = "".to_string();
        out = format!("{} SMART HOME: {}\n", out, self.name);
        for (name_room, room) in self.rooms {
            out = format!("{} Room Name: {}\n\t", out, name_room);
            for device in room.devices {
                out = format!("{} Device: {}\n\t", out, device);
            }
            out = format!("{}\n", out);
        }
        if out.is_empty() {
            return Err(SmartHouseErros::ReportIsEmpty);
        }
        Ok(out)
    }

    pub fn home_report_new<T: Formatter>(self, g: T) -> Result<SmartHouse, SmartHouseErros> {
        let mut out: String = "".to_string();
        g.format("smart_house", &self, &mut out);
        if out.is_empty() {
            return Err(SmartHouseErros::ReportIsEmpty);
        }
        Ok(self)
    }
}
