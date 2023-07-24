use test_patterns::static_v::{ChangeStatus, PrintStatus, SmartObject, Socket, Termometr};

fn main() {
    let mut socket = Socket::new();
    let mut termometr = Termometr::new();

    socket.accept(&PrintStatus);
    termometr.accept(&PrintStatus);

    socket.accept(&ChangeStatus);
    termometr.accept(&ChangeStatus);

    println!("Soket status: {0}", socket.get_report());
    println!("Termometer statis: {0}", termometr.get_report());
}
