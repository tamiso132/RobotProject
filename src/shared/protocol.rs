use std::ffi::c_float;
use std::ffi::c_int;
use std::mem::size_of;

use super::{read, write};
pub const HEADER: u16 = 0xAA | 0xAA << 8;

struct FloatCustom {
    hex_float: [u8; 4],
}
impl FloatCustom {
    fn to_float() -> c_float {
        return 1 as c_float; // TODO
    }
}

macro_rules! RESPONSE {
    ($struct_name_r:ident, $struct_name_s:ident,  {$($field_r:ident : $ty_r:ty),* }, {$($field_s:ident : $ty_s:ty),* }, $id:expr, $ctrl:expr) => {
        #[repr(C, packed)]
        struct  $struct_name_r {
            header: u16,
            len: u8,
            id: u8,
            ctrl: u8,
           $($field_r: $ty_r),*,
            checksum:u8,
        }
        #[repr(C, packed)]
        struct $struct_name_s {
            header: u16,
            len:u8,
            id:u8,
            ctrl:u8,
            $($field_s: $ty_s),*
            checksum:u8
        }

        impl $struct_name_s{
            fn new($($field_s: $ty_s),*) -> Self{
                let header = HEADER;
                let mut len:u8 = 0;
                let id = $id;
                let ctrl = $ctrl;
                len += 2;

                $(
                    len += std::mem::size_of::<$ty_s>();
                )*

                Self{
                    header,
                    len,
                    id,
                    ctrl,
                    $(
                    $field_s
                    )*
                    checksum:0,
                }
            }
        }
    };
}
RESPONSE!(GetPoseR, GetPoseS, {
    x: FloatCustom,
    y: FloatCustom,
    z: FloatCustom,
    r: FloatCustom,
    joint_angle: [FloatCustom; 4]
},{}, 10, 0);

unsafe fn any_as_u8_slice<T: Sized>(p: &T) -> &[u8] {
    ::core::slice::from_raw_parts((p as *const T) as *const u8, ::core::mem::size_of::<T>())
}

fn calculate_checksum(payload: &Vec<u8>) -> u8 {
    let mut amount: u8 = 0;
    for element in payload {
        amount += element;
    }
    return !amount + 1;
}

fn get_pose(fd: c_int) {
    let get_pose_command = GetPoseS::new();
    unsafe {
        let bytes = any_as_u8_slice(&get_pose_command);
        write(fd, bytes.to_vec());
        read(fd, size_of(), bytes)
    }
}
