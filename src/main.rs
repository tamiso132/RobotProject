use std::{
    thread::{self, Thread},
    time::Duration,
};

use robotproject::{
    self,
    cbinding::{self, close_port},
    protocol::robotprotocol::{self, FloatCustom},
};
fn main() {
    unsafe {
        let fd = cbinding::serial_open();
        robotprotocol::queue::ClearExec::send_immediate_command(fd);
        thread::sleep(Duration::from_millis(500));
        robotprotocol::homing::Param::send_queue_command(
            fd,
            &FloatCustom::new(200.0),
            &FloatCustom::new(200.0),
            &FloatCustom::new(200.0),
            &FloatCustom::new(200.0),
        );
        thread::sleep(Duration::from_millis(500));

        let vel: [FloatCustom; 4] = [
            FloatCustom::new(200.0),
            FloatCustom::new(200.0),
            FloatCustom::new(200.0),
            FloatCustom::new(200.0),
        ];

        let angle: [FloatCustom; 4] = [
            FloatCustom::new(200.0),
            FloatCustom::new(200.0),
            FloatCustom::new(200.0),
            FloatCustom::new(200.0),
        ];

        robotprotocol::ptp::Joint::send_queue_command(fd, &vel, &angle);
        thread::sleep(Duration::from_millis(500));

        robotprotocol::ptp::Common::send_queue_command(
            fd,
            &FloatCustom::new(100.0),
            &FloatCustom::new(100.0),
        );
        thread::sleep(Duration::from_millis(500));

        robotprotocol::homing::Cmd::send_get_command(0);
        thread::sleep(Duration::from_millis(500));
        robotprotocol::queue::StartExec::send_immediate_command(fd);
        thread::sleep(Duration::from_millis(500));
        close_port(fd);
        while true {}
    }
}
