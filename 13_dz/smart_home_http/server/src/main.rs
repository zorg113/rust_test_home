use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Header;
use rocket::serde::json::{json, Json, Value};
use rocket::serde::{Deserialize, Serialize};
use rocket::State;
use rocket::{get, routes};
use rocket::{Request, Response};
use serde_json;
use smart_home_lib::smart_devices::{
    DeviceChangeContentProvider, DeviceLocationProvider, RoomChangeContentProvider, SmartSocket,
    SmartThermometer,
};
use smart_home_lib::smart_house::{Formatter, SmartHouse};
use std::sync::{Arc, Mutex};
#[macro_use]
extern crate rocket;
#[derive(Serialize, Deserialize, Debug)]
enum Actions {
    Delete,
    Add,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
struct MessageRoom {
    action: Actions,
    name_room: String,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
struct MessageDevice {
    action: Actions,
    name_room: String,
    name_device: String,
}

impl RoomChangeContentProvider for MessageRoom {
    fn add_room_in_home(&self) -> &str {
        &self.name_room
    }

    fn delete_room(&self) -> &str {
        &self.name_room
    }
}

impl DeviceChangeContentProvider for MessageDevice {
    fn add_device_into_room(&self) -> (&str, &str) {
        (&self.name_room, &self.name_device)
    }

    fn delete_device_in_room(&self) -> (&str, &str) {
        (&self.name_room, &self.name_device)
    }
}

pub struct SmartHomeJson;

impl Formatter for SmartHomeJson {
    fn format(&self, key: &str, data: &SmartHouse, buf: &mut String) {
        buf.push_str(&json!({ key: data}).to_string())
    }
}

type SmartHouseHttp = Mutex<Box<SmartHouse>>;
type SmartHouseData<'a> = &'a State<Arc<SmartHouseHttp>>;

#[get("/smart_home/report")]
fn report(house: SmartHouseData<'_>) -> Value {
    let rep = house.lock().unwrap().clone();
    match rep.home_report_new(SmartHomeJson) {
        Ok(_u) => json!({"status": "ok", "data": &_u}),
        Err(e) => json!({"status": "err", "data": e.to_string()}),
    }
}

#[get("/smart_home/rooms")]
fn get_rooms(house: SmartHouseData<'_>) -> Value {
    let rooms = house.lock().unwrap();
    match rooms.get_rooms() {
        Ok(u) => json!({"status": "ok", "data": u}),
        Err(e) => json!({"status": "err", "data": e.to_string()}),
    }
}

#[get("/smart_home/<room>/devices")]
fn get_room_devices(room: String, house: SmartHouseData<'_>) -> Value {
    let dev = house.lock().unwrap();
    match dev.devices(&room) {
        Some(u) => json!({"status": "ok", "data": u}),
        None => json!({"status": "err"}),
    }
}

#[post("/smart_home/room", format = "json", data = "<message>")]
fn rooms(message: Json<MessageRoom>, house: SmartHouseData<'_>) -> Value {
    let mut h = house.lock().unwrap();
    let action = &message.action;
    let result = match action {
        Actions::Add => h.add_room(&message.0),
        Actions::Delete => h.delete_room(&message.0),
    };
    match result {
        Ok(_u) => json!({"status": "ok"}),
        Err(e) => json!({"status": "error" , "msg" : e.to_string()}),
    }
}

#[post("/smart_home/room/device", format = "json", data = "<message>")]
fn devices(message: Json<MessageDevice>, house: SmartHouseData<'_>) -> Value {
    let mut h = house.lock().unwrap();
    let action = &message.action;
    let result = match action {
        Actions::Add => h.add_device(&message.0),
        Actions::Delete => h.delete_device(&message.0),
    };
    match result {
        Ok(_u) => json!({"status": "ok"}),
        Err(e) => json!({"status": "error" , "msg" : e.to_string()}),
    }
}

#[catch(404)]
fn not_found() -> Value {
    json!({
        "status": "error",
        "reason": "Resource was not found."
    })
}

#[launch]
fn rocket() -> _ {
    let house = Arc::new(SmartHouseHttp::new(Box::new(SmartHouse::new(
        "MyTestHome".to_string(),
    ))));
    {
        let mut data = house.lock().unwrap();
        match data.add_devices(&load_devices()) {
            Ok(_) => (),
            Err(e) => panic!("Problem with add_device in home : {:?}", e),
        }
    }

    rocket::build()
        .attach(CORS)
        .mount("/api", routes![report, get_rooms, get_room_devices])
        .mount("/api", routes![rooms, devices])
        .register("/api", catchers![not_found])
        .manage(house)
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

pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "POST, GET, PATCH, OPTIONS",
        ));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}
