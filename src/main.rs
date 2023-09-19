// See the "macOS permissions note" in README.md before running this on macOS
// Big Sur or later.

use std::error::Error;
use std::time::Duration;
use tokio::time;

use btleplug::api::{Central, Manager as _, Peripheral};
use btleplug::platform::Manager;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let manager = Manager::new().await?;
    let adapter_list = manager.adapters().await?;
    if adapter_list.is_empty() {
        eprintln!("No Bluetooth adapters found");
    }

    for adapter in adapter_list.iter() {
        adapter.start_scan().await.unwrap();
        std::thread::sleep(time::Duration::from_secs(10));
        let periph = adapter.peripherals().await.unwrap();

        for p in periph {
            let device_name = p.properties().await.unwrap().unwrap().local_name.unwrap();

            if device_name == "tomtom" {
                p.connect().await.expect("error connecting");
            }
        }
    }
    Ok(())
}
