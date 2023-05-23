//use smart_home::smart_house::*;
use smart_home::smart_devices::*;

struct SmartKettle {
    //    pub status: String,
    pub name: String,
    pub room: String,
}

struct SmartLamp {
    //    pub status: String,
    pub name: String,
    pub room: String,
}

struct AllDeviceInfoProvider {
    lamps: Vec<SmartLamp>,
    kettles: Vec<SmartKettle>,
}

impl DeviceLocationProvider for AllDeviceInfoProvider {
    fn get_device_location_and_name(&self) -> Vec<(String, String)> {
        let mut out: Vec<(String, String)> = vec![];
        for sm_kettle in &self.kettles {
            out.push((sm_kettle.room.clone(), sm_kettle.name.clone()));
        }
        for sm_lamp in &self.lamps {
            out.push((sm_lamp.room.clone(), sm_lamp.name.clone()));
        }
        out
    }
}

#[cfg(test)]
mod test {
    use crate::{AllDeviceInfoProvider, SmartKettle, SmartLamp};

    #[test]
    fn name_smart_house() {
        let home = smart_home::smart_house::SmartHouse::new("MyHome".to_string());
        assert_eq!(home.get_name(), "MyHome".to_string())
    }
    #[test]
    fn add_one_device_in_room_smart_house() {
        let device: AllDeviceInfoProvider = AllDeviceInfoProvider {
            lamps: vec![SmartLamp {
                //status:"Off".to_string(),
                name: "Lamp1".to_string(),
                room: "Kitchen".to_string(),
            }],
            kettles: vec![],
        };
        let mut home = smart_home::smart_house::SmartHouse::new("MyHome".to_string());
        home.add_devices(&device);
        for room in home.get_rooms() {
            assert_eq!(*room, "Kitchen".to_string());
            let device = home.devices(room);
            assert_eq!(*device[0], "Lamp1".to_string());
        }
    }

    #[test]
    fn add_any_devices_in_different_rooms_smart_house() {
        let result = vec!["Kitchen".to_string(), "BedRoom".to_string()];
        let dev_ok = vec!["Lamp1".to_string(), "HotSpot".to_string()];
        let device: AllDeviceInfoProvider = AllDeviceInfoProvider {
            lamps: vec![SmartLamp {
                //status:"Off".to_string(),
                name: "Lamp1".to_string(),
                room: "BedRoom".to_string(),
            }],
            kettles: vec![SmartKettle {
                //status:"Off".to_string(),
                name: "HotSpot".to_string(),
                room: "Kitchen".to_string(),
            }],
        };
        let mut home = smart_home::smart_house::SmartHouse::new("MyHome".to_string());
        home.add_devices(&device);
        let rooms = home.get_rooms();
        assert!(rooms.iter().all(|item| result.contains(item)));
        let dev = vec![home.devices(rooms[0])[0],home.devices(rooms[1])[0]];
        assert!(dev.iter().all(|item| dev_ok.contains(item)));
    }

    #[test]
    fn add_any_devices_in_one_room_smart_house() {
        let dev_ok = vec!["Lamp1".to_string(), "HotSpot".to_string()];
        let device: AllDeviceInfoProvider = AllDeviceInfoProvider {
            lamps: vec![SmartLamp {
                //status:"Off".to_string(),
                name: "Lamp1".to_string(),
                room: "Kitchen".to_string(),
            }],
            kettles: vec![SmartKettle {
                //status:"Off".to_string(),
                name: "HotSpot".to_string(),
                room: "Kitchen".to_string(),
            }],
        };
        let mut home = smart_home::smart_house::SmartHouse::new("MyHome".to_string());
        home.add_devices(&device);
        let rooms = home.get_rooms();
        let dev = vec![home.devices(rooms[0])[0],home.devices(rooms[0])[0]];
        assert!(dev.iter().all(|item| dev_ok.contains(item)));
    }

    #[test]
    fn add_devices_with_equal_name_in_one_room_smart_house() {
        let device: AllDeviceInfoProvider = AllDeviceInfoProvider {
            lamps: vec![SmartLamp {
                //status:"Off".to_string(),
                name: "HotSpot".to_string(),
                room: "Kitchen".to_string(),
            }],
            kettles: vec![SmartKettle {
                //status:"Off".to_string(),
                name: "HotSpot".to_string(),
                room: "Kitchen".to_string(),
            }],
        };
        let mut home = smart_home::smart_house::SmartHouse::new("MyHome".to_string());
        assert!(!home.add_devices(&device))
    }
}
