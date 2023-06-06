use serde::{Serialize, Deserialize};
use trprot::client_trprot;
use trprot::errors_trprot;

#[derive(Serialize, Deserialize)]
enum Request{
    GetStatus,
    SetStatus,
}

#[derive(Serialize, Deserialize)]
enum Status {
    On,
    Off,
}

#[derive(Serialize, Deserialize)]
enum Message {
    Request { Id: Request, Value: Status,},
    Response {Id: Request, Value: Status,},
}

fn main() {
    let mes = Message::Request { Id: Request::GetStatus, Value: Status::Off};
    let msg = serde_json::to_string(&mes).unwrap();
    println!("Hello, world! {0} ", msg);
}
