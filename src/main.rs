use std::{
    thread::{self, Thread},
    time::Duration,
};

mod camera;

use robotproject::{
    self,
    cbinding::{self, close_port, read, write},
    protocol::{self, queue, FloatCustom, IntCustom},
};

fn main() {
    unsafe {
        //  let fd = cbinding::serial_open();

        camera::take_image();

        //  let pos = protocol::GetPoseR::send_immediate_command(fd).unwrap();

        // protocol::ptp::Cmd::send_immediate_command(
        //     fd,
        //     &protocol::ptp::PTPMode::MovlXYZ,
        //     &FloatCustom::new(200.0),
        //     &FloatCustom::new(0.0),
        //     &FloatCustom::new(0.0),
        //     &pos.r,
        // );
        // protocol::SuctionCup::send_immediate_command(fd, &0, &0);

        // protocol::EMotor::send_immediate_command(fd, &0, &1, &IntCustom::new(10000));
        // // [0x27, 0x10, 0x00, 0x00]

        // thread::sleep(Duration::from_millis(2000));
        // protocol::EMotor::send_immediate_command(fd, &0, &1, &IntCustom::new(0));

        //  protocol::GetPoseR::send_immediate_command(fd);
    }
}
