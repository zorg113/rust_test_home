use serde::{Deserialize, Serialize};
use std::error::Error;
use std::process;
use trprot::server_trprot::{TrprotConnection, TrprotServer};

struct SmartSocket {
    status: Status,
    power: f32,
}

#[derive(Serialize, Deserialize)]
enum Request {
    GetStatus,
    SetStatus,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Copy)]
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

fn main() {
    ctrlc::set_handler(move || {
        println!("SmartSocket stop (received Ctrl+C!)");
        process::exit(0)
    })
    .expect("Error setting Ctrl-C handler");
    let mut sm_socket = SmartSocket::new();
    let server = TrprotServer::bind("127.0.0.1:55331").unwrap();
    println!("Start SmartSocket ... (Ctrl+C exit)");
    for connection in server.incoming() {
        process_connection(connection.unwrap(), &mut sm_socket).unwrap();
    }
    print!("End server");
}

fn process_connection(
    mut conn: TrprotConnection,
    sm_socket: &mut SmartSocket,
) -> Result<(), Box<dyn Error>> {
    let req = conn.recv_request()?;
    println!("Log: request {0}", req);
    let mesg: Message = serde_json::from_str(&req).unwrap();
    let (_, status) = match mesg {
        Message::Request { id, value } => (id, value),
        _ => panic!("logical error"),
    };
    sm_socket.set_status(status);
    let response = sm_socket.get_status();
    conn.send_response(serde_json::to_string(&response)?)?;
    Ok(())
}

impl SmartSocket {
    fn new() -> SmartSocket {
        SmartSocket {
            status: Status::Off,
            power: 0.,
        }
    }

    fn set_status(&mut self, status: Status) -> Option<bool> {
        match status {
            Status::On => {
                self.status = Status::On;
                self.power = 223.;
            }
            Status::Off => {
                self.status = Status::Off;
                self.power = 0.
            }
            Status::None => return None,
        }
        Some(true)
    }

    fn get_status(&self) -> Message {
        Message::Response {
            id: Request::GetStatus,
            value: self.status,
            power: self.power,
        }
    }
}
