use smart_home::smart_devices::*;
use smart_home::smart_house_errors::*;

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

struct DeviceContent {
    room: String,
    dev: String,
}

impl DeviceChangeContentProvider for DeviceContent {
    fn add_device_into_room(&self) -> (&str, &str) {
        (&self.room, &self.dev)
    }

    fn add_room_in_home(&self) -> &str {
        &self.room
    }

    fn delete_device_in_room(&self) -> (&str, &str) {
        (&self.room, &self.dev)
    }

    fn delete_room(&self) -> &str {
        &self.room
    }
}

#[cfg(test)]
mod test {
    use crate::{AllDeviceInfoProvider, DeviceContent, SmartHouseErros, SmartKettle, SmartLamp};

    macro_rules! assert_err {
        ($expression:expr, $($pattern:tt)+) => {
            match $expression {
                $($pattern)+ => (),
                ref e => panic!("expected `{}` but got `{:?}`", stringify!($($pattern)+), e),
            }
        }
    }

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
        match home.add_devices(&device) {
            Ok(u) => u,
            Err(e) => panic!("Problem with add device : {:?}", e),
        };
        let rooms = match home.get_rooms() {
            Ok(r) => r,
            Err(e) => panic!("Problem with get rooms : {:?}", e),
        };
        for room in rooms {
            assert_eq!(*room, "Kitchen".to_string());
            let device = match home.devices(room) {
                Some(u) => u,
                None => break,
            };
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
        match home.add_devices(&device) {
            Ok(u) => u,
            Err(e) => panic!("Problem with add device : {:?}", e),
        };
        let rooms = match home.get_rooms() {
            Ok(r) => r,
            Err(_) => panic!("Problem with get rooms"),
        };
        assert!(rooms.iter().all(|item| result.contains(item)));
        let devices_first = match home.devices(rooms[0]) {
            Some(vec) => vec,
            None => return,
        };
        let devices_second = match home.devices(rooms[1]) {
            Some(vec) => vec,
            None => return,
        };
        let dev = vec![devices_first[0], devices_second[0]];
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
        match home.add_devices(&device) {
            Ok(u) => u,
            Err(e) => panic!("Problem with add device : {:?}", e),
        };
        let rooms = match home.get_rooms() {
            Ok(r) => r,
            Err(e) => panic!("Problem with get rooms : {:?}", e),
        };
        let devices_first = match home.devices(rooms[0]) {
            Some(vec) => vec,
            None => return,
        };
        let dev = vec![devices_first[0], devices_first[0]];
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
        let _msg = "HotSpot".to_string();
        assert_err!(
            home.add_devices(&device),
            Err(SmartHouseErros::AddNotUniqueDeviceInRoom(_msg))
        );
    }

    #[test]
    fn change_content_in_home() {
        let add_device1: DeviceContent = DeviceContent {
            room: "Kitchen".to_string(),
            dev: "Socket".to_string(),
        };
        let add_device2: DeviceContent = DeviceContent {
            room: "Kitchen".to_string(),
            dev: "Multicooker".to_string(),
        };
        let add_device3: DeviceContent = DeviceContent {
            room: "Cabinet".to_string(),
            dev: "Lamp".to_string(),
        };
        let mut home = smart_home::smart_house::SmartHouse::new("TestHome".to_string());
        home.add_device(&add_device1).unwrap();
        home.add_device(&add_device2).unwrap();
        let rez = home.devices("Kitchen");

        if let Some(mut x) = rez {
            x.sort();
            assert_eq!(x, vec![&"Multicooker".to_string(), &"Socket".to_string()]);
        }

        home.delete_device(&add_device2).unwrap();
        let rez = home.devices("Kitchen");
        if let Some(mut x) = rez {
            x.sort();
            assert_eq!(x, vec![&"Socket".to_string()]);
        }

        home.add_room(&add_device3).unwrap();

        let rez = home.get_rooms();
        if let Ok(mut x) = rez {
            x.sort();
            assert_eq!(x, vec![&"Cabinet".to_string(), &"Kitchen".to_string()]);
        }

        home.delete_room(&add_device3).unwrap();
        let rez = home.get_rooms();

        if let Ok(mut x) = rez {
            x.sort();
            assert_eq!(x, vec![&"Kitchen".to_string()]);
        }
    }
}
