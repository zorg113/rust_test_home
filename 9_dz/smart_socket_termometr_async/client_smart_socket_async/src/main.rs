use serde::{Deserialize, Serialize};
use std::error::Error;
use trprot::client_trprot::TrprotClient;

#[derive(Serialize, Deserialize)]
enum Request {
    GetStatus,
    SetStatus,
}

#[derive(Serialize, Deserialize)]
enum Status {
    On,
    Off,
    None,
}

#[derive(Serialize, Deserialize)]
enum Message {
    Request {
        id: Request,
        value: Status,
    },
    Response {
        id: Request,
        value: Status,
        power: f32,
    },
}

fn main() -> Result<(), Box<dyn Error>> {
    let mesg_get_status = Message::Request {
        id: Request::GetStatus,
        value: Status::None,
    };
    let mesg_on = Message::Request {
        id: Request::SetStatus,
        value: Status::On,
    };
    let mesg_off = Message::Request {
        id: Request::SetStatus,
        value: Status::Off,
    };
    // message get status smart_socket
    send_command(mesg_get_status)?;
    // message on smart_socket
    send_command(mesg_on)?;
    // message off smart_socket
    send_command(mesg_off)?;
    Ok(())
}

fn send_command(msg: Message) -> Result<(), Box<dyn Error>> {
    let mut client = TrprotClient::connect("127.0.0.1:55331")?;
    let mesg = serde_json::to_string(&msg).unwrap();
    let response = client.send_request(mesg)?;
    println!("Log: SmartSocket send {0}", response);
    Ok(())
}
