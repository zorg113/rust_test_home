use std::string;
use std::vec::Vec;
use std::collections::HashMap;
// Метка todo - реализовать самостоятельно

// ***** Пример библиотеки "Умный дом" со статическим содержимым

trait Device{
    fn GetName(&self)-> String;
    fn GetType(&self)-> String;
}

struct Room {
     name: String,
     devices : Vec< Box<dyn Device> >, 
}

struct SmartHouse {
        name: String,
        rooms: Vec<Room>, 
}

impl SmartHouse {
    fn new() -> Self {
        let name = String::from("White house");
        let mut rooms: Vec<Room>;
        rooms.push(Room { name: String::from("Bedroom")
            , devices: vec![Box::new(SmartSocket{name: String::from("Var1")})] });
        SmartHouse {
            name : name,
            rooms: rooms,
        }
        //todo!("реализовать инициализацию дома")
    }

    fn get_rooms(&self) -> [&str; 2] {
        // Размер возвращаемого массива можно выбрать самостоятельно
        todo!("список комнат")
    }

    fn devices(&self, room: &str) -> [&str; 3] {
        // Размер возвращаемого массива можно выбрать самостоятельно
        todo!("список устройств в комнате `room`")
    }

    fn create_report(
        &self, 
        /* todo: принять обобщённый тип предоставляющий информацию об устройствах */
    ) -> String {
        todo!("перебор комнат и устройств в них для составления отчёта")
    }
}

trait DeviceInfoProvider {
    // todo: метод, возвращающий состояние устройства по имени комнаты и имени устройства
}

// ***** Пример использования библиотеки умный дом:

// Пользовательские устройства:
struct SmartSocket {
    name: String,
    type_dev: String,
    status:String,
}

impl Device for SmartSocket{
    fn GetName(&self)-> String {
        self.name
    }
    fn GetType(&self)-> String{
        self.type_dev
    }
}
struct SmartThermometer {
    name:String,
    type_dev: String,
    status:String,
}

impl Device for SmartThermometer{
    fn GetName(&self)-> String {
        self.name
    }
    fn GetType(&self)-> String{
        self.type_dev
    }
}



// Пользовательские поставщики информации об устройствах.
// Могут как хранить устройства, так и заимствывать.
struct OwningDeviceInfoProvider {
    socket: SmartSocket,
}
struct BorrowingDeviceInfoProvider<'a, 'b> {
    socket: &'a SmartSocket,
    thermo: &'b SmartThermometer,
}


// todo: реализация трейта `DeviceInfoProvider` для поставщиков информации

fn main() {
    // Инициализация устройств
    let socket1 = SmartSocket {};
    let socket2 = SmartSocket {};
    let thermo = SmartThermometer {};

    // Инициализация дома
    let house = SmartHouse::new();


    // Строим отчёт с использованием `OwningDeviceInfoProvider`.
    let info_provider_1 = OwningDeviceInfoProvider {
        socket: socket1,
    };
    // todo: после добавления обобщённого аргумента в метод, расскоментировать передачу параметра
    let report1 = house.create_report(/* &info_provider_1 */);

    // Строим отчёт с использованием `BorrowingDeviceInfoProvider`.
    let info_provider_2 = BorrowingDeviceInfoProvider {
        socket: &socket2,
        thermo: &thermo,
    };
    // todo: после добавления обобщённого аргумента в метод, расскоментировать передачу параметра
    let report2 = house.create_report(/* &info_provider_2 */);

    // Выводим отчёты на экран:
    println!("Report #1: {report1}");
    println!("Report #2: {report2}");
}
