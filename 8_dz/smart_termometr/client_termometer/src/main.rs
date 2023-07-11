use std::{thread, time::Duration};

use trprot_udp::{Buffer, SmartThermo};

fn main() {
    let receiver_address = "127.0.0.1:4321";
    //let thermo = SmartThermo::new(receiver_address).unwrap();
    let mut buff = Buffer::new();
    let a: f64 = 236.36;
    let mesg = String::from("ABCDEF");
    buff.write_message(a.to_be_bytes().to_vec());
    buff.write_message(mesg.into_bytes());
    buff.print();
    let m = buff.read_message();
    println!("{}", m)
    //for _ in 0..120 {
    //    thread::sleep(Duration::from_secs(1));
    //    let temperature = thermo.get_temperature();
    //    println!("The temperature is {temperature}");
    //}
}
