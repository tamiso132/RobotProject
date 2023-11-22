use std::{
    ffi::CString,
    process::Command,
    thread::{self, Thread},
    time::Duration,
};

use robotproject::{
    self,
    cbinding::{self, close_port, read, write},
    protocol::{self, queue, sensor, FloatCustom, IntCustom, SuctionCup},
};

pub fn take_picture() {
    let output = Command::new("libcamera-jpeg")
    .arg("-o")
    .arg("/home/tom/projects/RobotProject/src/ty.jpg")
    .output()
    .expect("Failed to execute libcamera-still command");

if !output.status.success() {
    eprintln!("Error: {}", String::from_utf8_lossy(&output.stderr));
}

}

fn main() {
    unsafe {
        // let s = String::from("HalloWelt!");
        // let cs = CString::new(s).unwrap();
        // let cv: Vec<u8> = cs.into_bytes_with_nul();
        // let mut tmp: Vec<i8> = cv.into_iter().map(|c| c as i8).collect::<_>(); // line 7
        // let _cptr: *mut i8 = tmp.as_mut_ptr();

        // cbinding::bindings::takee_pic(_cptr);

        let fd = cbinding::serial_open();

        // sensor::set_infrared_immediate(fd, 1, sensor::Port::GP4);

        // protocol::EMotor::send_immediate_command(fd, &0, &1, &IntCustom::new(10000));

        // thread::sleep(Duration::from_millis(2000));
        // protocol::EMotor::send_immediate_command(fd, &0, &0, &IntCustom::new(10000));
        sensor::get_infrared_state(fd, 0);
        //  loop {
        //       println!("State: {}", sensor::get_infrared_state(fd, 0) as u8);
        //       thread::sleep(Duration::from_millis(1000));
        //  }

        // protocol::SuctionCup::send_immediate_command(fd, &1, &1);

        // thread::sleep(Duration::from_millis(2000));
        // protocol::SuctionCup::send_immediate_command(fd, &0, &0);

        // protocol::EMotor::send_immediate_command(fd, &0, &1, &IntCustom::new(10000));

        // thread::sleep(Duration::from_millis(3000));
        // protocol::EMotor::send_immediate_command(fd, &0, &1, &IntCustom::new(0));

        // queue::StopExec::send_immediate_command(fd);
        // queue::ClearExec::send_immediate_command(fd);
        // let mut last_index = 0;
        // let mut f = FloatCustom::new(0.0);
        // let mut y_add = 10.0;

        // for i in 0..5 {
        //     y_add += 20.0;
        //     last_index = protocol::ptp::Cmd::send_queue_command(
        //         fd,
        //         &protocol::ptp::PTPMode::MovlXYZ,
        //         &FloatCustom::new(175.0),
        //         &FloatCustom::new(0.0),
        //         &FloatCustom::new(0.0),
        //         &pos.r,
        //     )
        //     .unwrap();
        // }
        let mut pos = protocol::GetPoseR::send_immediate_command(fd).unwrap();

        queue::ClearExec::send_immediate_command(fd);
        queue::StopExec::send_immediate_command(fd);

        protocol::homing::Cmd::send_queue_command(fd, &0);
        println!("x: {}", &pos.x.to_float());
         println!("y: {}", &pos.y.to_float());
          println!("z: {}", &pos.z.to_float());
        protocol::ptp::Cmd::send_queue_command(
            fd,
            &protocol::ptp::PTPMode::MovlXYZ,
            &FloatCustom::new(100.0),
            &FloatCustom::new(0.0),
            &FloatCustom::new(0.0),
            &pos.r,
        );

        queue::CurrentIndex::send_immediate_command(fd, current_index)
        protocol::queue::StartExec::send_immediate_command(fd);

        // queue::StartExec::send_immediate_command(fd);

        // while protocol::queue::CurrentIndex::send_get_command(fd)
        //     .unwrap()
        //     .current_index
        //     >= last_index
        // {}
    }
}
