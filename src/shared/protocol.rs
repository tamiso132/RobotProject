use std::ffi::c_float;
use std::ffi::c_int;

use super::{read, write};
pub const HEADER: [u8; 2] = [0xAA, 0xAA];

macro_rules! RESPONSE {
    ($name:ident, { $($field:ident : $ty:ty),* }) => {
        #[repr(C)]
        struct $name {
            header: u16,
            len: u8,
            id: u8,
            ctrl: u8,
            $($field: $ty),*
            checksum:u8,
        }
    };
}
RESPONSE!(GetPoseResponse,
{
    x:c_float,
    y:c_float,
    z:c_float,
    r:c_float,
    joint_angles:[c_float;4]
});

fn calculate_checksum(payload: &Vec<u8>) -> u8 {
    let mut amount: u8 = 0;
    for element in payload {
        amount += element;
    }
    return !amount + 1;
}

fn get_pose(fd: c_int) {
    let mut payload = vec![0x0A, 0x00];
    let len = payload.len() as u8;
    let checksum = calculate_checksum(&payload);

    let mut command: Vec<u8> = vec![];

    command.extend(HEADER);
    command.push(len);
    command.append(&mut payload);
    command.push(checksum);
    let mut bytes;
    unsafe {
        write(fd, command);

        let bytes_read = read(fd, 256, bytes);
    }
}
