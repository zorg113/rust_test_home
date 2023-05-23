mod smart_socket;
mod smart_termometr;

fn main() {
    let smsocket = smart_socket::SmartSocket::new();
    println!(
        "SmartSocket description: {} On/Off: {} Power: {}",
        smsocket.description(),
        smsocket.on_off(),
        smsocket.power()
    );
    let smterm = smart_termometr::SmartTermometr::new();
    println!("SmartTermometr: {}", smterm.temp());
}
