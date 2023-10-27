use std::ptr;

use robotproject::shared;
fn main() {
    unsafe {
        // let fd = shared::serial_open();
        // let mut ret = shared::protocol::GetPoseR::send_immediate_command(fd).unwrap();

        // let header_1 = ret.header as u8;
        // let header_2 = ret.header >> 8;
        // let id = ret.id;
        // let len = ret.len;
        // let checksum = ret.checksum;
        // ret.joint_angle[0].to_float();

        // println!("ctrl; {}", ret.joint_angle[0].to_float());
        shared::protocol::generate(10, 15);
    }
}
