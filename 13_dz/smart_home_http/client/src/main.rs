// tokio let's us use "async" on our main function
#[tokio::main]
async fn main() {
    let client = reqwest::Client::new();
    // Get Home Report
    let result = client
        .get("http://127.0.0.1:8000/api/smart_home/report")
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    println!("** Home Report {}", result);
    println!(" ------------- ");
    // Get Rooms in home
    let result = client
        .get("http://127.0.0.1:8000/api/smart_home/rooms")
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    println!("** Home rooms {}", result);
    println!(" ------------- ");
    // Add Room
    let json_data = r#"{ "action": "Add", "name_room": "TestRoom"}"#;
    let result = client
        .post("http://127.0.0.1:8000/api/smart_home/room")
        .header("Content-Type", "application/json")
        .body(json_data.to_owned())
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    println!("** Home Add room {}", result);
    let result = client
        .get("http://127.0.0.1:8000/api/smart_home/rooms")
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    println!("** Home rooms {}", result);
    println!(" ------------- ");
    // Delete Room
    let json_data = r#"{ "action": "Delete", "name_room": "TestRoom"}"#;
    let result = client
        .post("http://127.0.0.1:8000/api/smart_home/room")
        .header("Content-Type", "application/json")
        .body(json_data.to_owned())
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    println!("** Home Delete room {}", result);
    let result = client
        .get("http://127.0.0.1:8000/api/smart_home/rooms")
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    println!("** Home rooms {}", result);
    println!(" ------------- ");
    // Add  Device in Room
    let json_data = r#" { "action": "Add", "name_room": "Kitchen", "name_device": "boiler"}"#;
    let result = client
        .post("http://127.0.0.1:8000/api/smart_home/room/device")
        .header("Content-Type", "application/json")
        .body(json_data.to_owned())
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    println!("** Home Add Device in Kitchen {}", result);
    let result = client
        .get("http://127.0.0.1:8000/api/smart_home/Kitchen/devices")
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    println!("** Home Kitchen devices {}", result);
    println!(" ------------- ");
    // Delete Device in Room
    let json_data = r#" { "action": "Delete", "name_room": "Kitchen", "name_device": "boiler"}"#;
    let result = client
        .post("http://127.0.0.1:8000/api/smart_home/room/device")
        .header("Content-Type", "application/json")
        .body(json_data.to_owned())
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    println!("** Home Delete Device in Kitchen {}", result);
    let result = client
        .get("http://127.0.0.1:8000/api/smart_home/Kitchen/devices")
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    println!("** Home Kitchen devices {}", result);
    println!(" ------------- ");
    // Report by Status Device
    let result = client
        .get("http://127.0.0.1:8000/api//smart_home/device_status")
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    println!("** Home Report with device status {}", result);
    println!(" ------------- ");
}
