use smart_home::smart_devices::*;
use smart_home::smart_house::*;
use smart_home::smart_house_errors::DeviceInfoProviderError;
// ***** Пример использования библиотеки умный дом
// Пользовательские поставщики информации об устройствах.
// Могут как хранить устройства, так и заимствывать.

struct OwningDeviceInfoProvider {
    socket: SmartSocket,
}
struct BorrowingDeviceInfoProvider<'a, 'b> {
    socket: &'a SmartSocket,
    thermo: &'b SmartThermometer,
}

struct AllDeviceInfoProvider {
    sockets: Vec<SmartSocket>,
    thermo: Vec<SmartThermometer>,
}

impl DeviceLocationProvider for AllDeviceInfoProvider {
    fn get_device_location_and_name(&self) -> Vec<(String, String)> {
        let mut out: Vec<(String, String)> = vec![];
        for sm_socket in &self.sockets {
            out.push((sm_socket.room.clone(), sm_socket.name.clone()));
        }
        for sm_therm in &self.thermo {
            out.push((sm_therm.room.clone(), sm_therm.name.clone()));
        }
        out
    }
}

impl DeviceInfoProvider for OwningDeviceInfoProvider {
    fn get_device_info(&self, room: &str, name: &str) -> Result<String, DeviceInfoProviderError> {
        let mut out: String = "".to_string();
        if self.socket.room == *room && self.socket.name == *name {
            out = format!(
                "Device Name: {}  Device Status: {} ",
                name, self.socket.status
            );
        }
        Ok(out)
    }
}

impl<'a, 'b> DeviceInfoProvider for BorrowingDeviceInfoProvider<'a, 'b> {
    fn get_device_info(&self, room: &str, name: &str) -> Result<String, DeviceInfoProviderError> {
        let mut out: String = "".to_string();
        if self.socket.room == *room && self.socket.name == *name {
            out = format!(
                "Device Name: {} Device Status: {}",
                name, self.socket.status
            );
        }
        if self.thermo.room == *room && self.thermo.name == *name {
            out = format!(
                "Device Name: {} Device Status: {}",
                name, self.thermo.status
            );
        }
        Ok(out)
    }
}

fn load_devices() -> AllDeviceInfoProvider {
    let devices_in_home: AllDeviceInfoProvider = AllDeviceInfoProvider {
        sockets: vec![
            SmartSocket {
                name: "SmartSocket_Kitchen_N1".to_string(),
                room: "Kitchen".to_string(),
                status: "Off".to_string(),
            },
            SmartSocket {
                name: "SmartSocket_Kitchen_N2".to_string(),
                room: "Kitchen".to_string(),
                status: "Off".to_string(),
            },
            SmartSocket {
                name: "SmartSocket_Kitchen_N3".to_string(),
                room: "Kitchen".to_string(),
                status: "Off".to_string(),
            },
            SmartSocket {
                name: "SmartSocket_BedRoom_N1".to_string(),
                room: "BedRoom".to_string(),
                status: "Off".to_string(),
            },
            SmartSocket {
                name: "SmartSocket_BedRoom_N2".to_string(),
                room: "BedRoom".to_string(),
                status: "Off".to_string(),
            },
        ],
        thermo: vec![
            SmartThermometer {
                name: "SmartThermometr_Kitchen_N1".to_string(),
                room: "Kitchen".to_string(),
                status: "Off".to_string(),
            },
            SmartThermometer {
                name: "SmartThermometr_BedRoom_N1".to_string(),
                room: "BedRoom".to_string(),
                status: "Off".to_string(),
            },
            SmartThermometer {
                name: "SmartThermometr_BedRoom_N2".to_string(),
                room: "BedRoom".to_string(),
                status: "Off".to_string(),
            },
        ],
    };
    devices_in_home
}

fn main() {
    // Инициализация устройств
    let socket1 = SmartSocket {
        name: "SmartSocket_Kitchen_N1".to_string(),
        room: "Kitchen".to_string(),
        status: "On".to_string(),
    };
    let socket2 = SmartSocket {
        name: "SmartSocket_BedRoom_N1".to_string(),
        room: "BedRoom".to_string(),
        status: "Off".to_string(),
    };
    let thermo = SmartThermometer {
        name: "SmartThermometr_BedRoom_N1".to_string(),
        room: "BedRoom".to_string(),
        status: "Off".to_string(),
    };
    // Display
    println!("Display SmartSocket => {} ", socket2);
    println!("Display SmartTermometer => {} ", thermo);
    // Инициализация дома и передача спика устройств дому
    let mut house = SmartHouse::new("MyHome".to_string());
    match house.add_devices(&load_devices()) {
        Ok(_) => (),
        Err(e) => panic!("Problem with add_device in home : {:?}", e),
    }
    // Строим отчёт с использованием `OwningDeviceInfoProvider`.
    let info_provider_1 = OwningDeviceInfoProvider { socket: socket1 };

    let report1 = match house.create_report(&info_provider_1) {
        Ok(u) => u,
        Err(e) => panic!("Problem with report 1 : {:?}", e),
    };

    // Строим отчёт с использованием `BorrowingDeviceInfoProvider`.
    let info_provider_2 = BorrowingDeviceInfoProvider {
        socket: &socket2,
        thermo: &thermo,
    };

    let report2 = match house.create_report(&info_provider_2) {
        Ok(u) => u,
        Err(e) => panic!("Problem with report 2 : {:?}", e),
    };
    let home_report = match house.home_report() {
        Ok(u) => u,
        Err(e) => panic!("Problem with global home report : {:?}", e),
    };
    // Выводим отчёты на экран:
    println!("Report #1: {report1}");
    println!("Report #2: {report2}");
    println!("Report #3:\n{home_report}");
}
