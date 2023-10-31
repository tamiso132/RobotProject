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

       // robotprotocol::SuctionCup::send_immediate_command(fd, &1, &1);
        robotprotocol::queue::StopExec::send_immediate_command(fd);
        robotprotocol::queue::ClearExec::send_immediate_command(fd);
        robotprotocol::queue::StopExec::send_immediate_command(fd);
        let velocity = [
            FloatCustom::new(200.0),
            FloatCustom::new(200.0),
            FloatCustom::new(200.0),
            FloatCustom::new(200.0),
        ];
        let acceleration = [
            FloatCustom::new(200.0),
            FloatCustom::new(200.0),
            FloatCustom::new(200.0),
            FloatCustom::new(200.0),
        ];

        println!("C now");
        let c =
            robotprotocol::ptp::Joint::send_queue_command(fd, &velocity, &acceleration).unwrap();

        println!("First {}", c);
        let d = robotprotocol::ptp::Coordinate::send_queue_command(
            fd,
            &FloatCustom::new(200.0),
            &FloatCustom::new(200.0),
            &FloatCustom::new(200.0),
            &FloatCustom::new(200.0),
        )
        .unwrap();
        println!("Second {}", d);
        robotprotocol::ptp::Jump::send_queue_command(
            fd,
            &FloatCustom::new(10.0),
            &FloatCustom::new(200.0),
        );
        println!("Third");
        robotprotocol::ptp::Common::send_queue_command(
            fd,
            &FloatCustom::new(100.0),
            &FloatCustom::new(100.0),
        );
        println!("Fourth");
        let pos = robotprotocol::GetPoseR::send_immediate_command(fd).unwrap();
        println!("Fifth");

        let mut pos_x = pos.x;
        let mut pos_y = pos.y;
        let mut pos_z = pos.z;
        let mut r_head = pos.r;

        let mut x = FloatCustom::new(0.0);
        let mut y = FloatCustom::new(0.0);
        let mut z = FloatCustom::new(0.0);

        let mut last_index = 0;

        for i in 0..4 {
            last_index = robotprotocol::ptp::Cmd::send_queue_command(
                fd,
                &robotprotocol::ptp::PTPMode::MovlXYZ,
                &FloatCustom::new(x.to_float() + pos_x.to_float()),
                &FloatCustom::new(y.to_float() + pos_y.to_float()),
                &FloatCustom::new(z.to_float() + pos_z.to_float()),
                &FloatCustom::new(x.to_float() + pos_x.to_float()),
            )
            .unwrap();
            println!("China number 2 {}", last_index);
            pos_x = FloatCustom::new(pos_x.to_float() + 10.0);
        }
        robotprotocol::queue::StartExec::send_immediate_command(fd);
        let mut curr = robotprotocol::queue::CurrentIndex::send_get_command(fd)
            .unwrap()
            .current_index;
        while curr < last_index {
            thread::sleep(Duration::from_millis(500));

            println!(
                "Still working hard, \ncurrent index: {}\nlast index: {}",
                curr, last_index
            );
            curr = robotprotocol::queue::CurrentIndex::send_get_command(fd)
                .unwrap()
                .current_index;
        }
    }
}
