use std::{
    ffi::CString,
    thread::{self, Thread},
    time::Duration,
};

use robotproject::{
    self,
    cbinding::{self, close_port, read, write},
    protocol::{self, queue, FloatCustom, IntCustom},
};

fn main() {
    unsafe {
        let s = String::from("HalloWelt!");
        let cs = CString::new(s).unwrap();
        let cv: Vec<u8> = cs.into_bytes_with_nul();
        let mut tmp: Vec<i8> = cv.into_iter().map(|c| c as i8).collect::<_>(); // line 7
        let _cptr: *mut i8 = tmp.as_mut_ptr();

        cbinding::bindings::takee_pic(_cptr);

        //  let fd = cbinding::serial_open();

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
