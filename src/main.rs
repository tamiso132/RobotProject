use std::{
    thread::{self, Thread},
    time::Duration,
};

use robotproject::{
    self,
    cbinding::{self, close_port, read, write},
    protocol::robotprotocol::{self, FloatCustom},
};
fn main() {
    unsafe {
        let fd = cbinding::serial_open();

        let x = FloatCustom::new(200.0);
        let y = FloatCustom::new(200.0);
        let z = FloatCustom::new(200.0);
        let r = FloatCustom::new(200.0);

        let ax = [
            FloatCustom::new(200.0),
            FloatCustom::new(200.0),
            FloatCustom::new(200.0),
            FloatCustom::new(200.0),
        ];

        robotprotocol::ptp::Joint::send_immediate_command(fd, &[x, y, z, r], &ax);
        robotprotocol::ptp::Coordinate::send_immediate_command(
            fd,
            &FloatCustom::new(200.0),
            &FloatCustom::new(200.0),
            &FloatCustom::new(200.0),
            &r,
        );
        robotprotocol::ptp::Jump::send_immediate_command(
            fd,
            &FloatCustom::new(10.0),
            &FloatCustom::new(200.0),
        );
        robotprotocol::ptp::Common::send_immediate_command(
            fd,
            &FloatCustom::new(100.0),
            &FloatCustom::new(100.0),
        );

        let pose = robotprotocol::GetPoseR::send_immediate_command(fd).unwrap();

        let mut xx = pose.x;
        let mut yy = pose.y;
        let mut zz = pose.z;
        let mut rr = pose.r;

        let mut mov_x = 0.0;
        let mut mov_y = 0.0;
        let mut mov_z = 10.0;
        let mut mov_flag = -1.0;

        loop {
            mov_flag *= -1.0;

            for i in 0..5 {
                robotprotocol::ptp::Cmd::send_immediate_command(
                    fd,
                    &robotprotocol::ptp::PTPMode::MovlXYZ,
                    &FloatCustom::new(xx.to_float() + mov_x),
                    &FloatCustom::new(yy.to_float() + mov_y),
                    &FloatCustom::new(zz.to_float() + mov_z),
                    &FloatCustom::new(rr.to_float()),
                );

                mov_x += 10.0 * mov_flag;

                robotprotocol::ptp::Cmd::send_immediate_command(
                    fd,
                    &robotprotocol::ptp::PTPMode::MovlXYZ,
                    &FloatCustom::new(xx.to_float() + mov_x),
                    &FloatCustom::new(yy.to_float() + mov_y),
                    &FloatCustom::new(zz.to_float() + mov_z),
                    &FloatCustom::new(rr.to_float()),
                );
                robotprotocol::ptp::Cmd::send_immediate_command(
                    fd,
                    &robotprotocol::ptp::PTPMode::MovlXYZ,
                    &FloatCustom::new(xx.to_float() + mov_x.clone()),
                    &FloatCustom::new(yy.to_float() + mov_y),
                    &FloatCustom::new(zz.to_float()),
                    &FloatCustom::new(rr.to_float()),
                );
            }
        }
    }
}
