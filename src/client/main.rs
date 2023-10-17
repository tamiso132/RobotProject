use std::ptr;

use robotproject::shared;
fn main() {
    unsafe {
        let fd = shared::serial_open();
        let ret = shared::protocol::GetPoseR::send_command(fd).unwrap();
        let test = ret.id;
        println!("header: {:?}", test);
    }
}
