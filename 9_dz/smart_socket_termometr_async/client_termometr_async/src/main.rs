use std::time::Duration;
use tokio::time;

use trprot_udp_async::SmartThermo;

#[tokio::main]
async fn main() {
    let receiver_address = "127.0.0.1:4321";
    let thermo = SmartThermo::new(receiver_address).await.unwrap();
    for _ in 0..120 {
        time::sleep(Duration::from_secs(1)).await;
        let temperature = thermo.get_temperature().await;
        println!("The temperature is {temperature}");
    }
}
