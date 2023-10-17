use std::ffi::c_float;
use std::ffi::c_int;
use std::mem::size_of;

use super::{read, write};
pub const HEADER: u16 = 0xAA | 0xAA << 8;

#[repr(C, packed)]
#[derive(Default)]
pub struct FloatCustom {
    hex_float: [u8; 4],
}
impl FloatCustom {
    fn to_float() -> c_float {
        return 1 as c_float; // TODO
    }
}

macro_rules! RESPONSE {
    ($struct_name_r:ident, {$($field_r:ident : $ty_r:ty),* }, {$($field_s:ident : $ty_s:ty),* }, $id:expr, $ctrl:expr) => {
        #[repr(C, packed)]
        #[derive(Default)]
        pub struct  $struct_name_r {
            pub header: u16,
            pub len: u8,
            pub id: u8,
            pub ctrl: u8,
           $(pub $field_r: $ty_r),*,
            pub checksum:u8,
        }

        impl $struct_name_r{
            pub fn send_command($($field_s: $ty_s),* fd:c_int) -> Option<$struct_name_r>{

                let mut len:u8 = 2;
                let mut checksum:u8 = 0;
                checksum += $id;
                checksum += $ctrl;

                println!("hello");

                let mut send_packet:Vec<u8> = vec!();

                $(
                    len += std::mem::size_of::<$ty_s>();
                )*

                println!("hello");
                send_packet.push(HEADER as u8);
                send_packet.push(HEADER as u8);
                send_packet.push(len);
                send_packet.push($id as u8);
                send_packet.push($ctrl as u8);
                println!("hello");
                println!("hello");
                $(
                    let array = any_as_u8_slice($field_s);
                    for(let i  = 0; i < array.len(); i++){
                        send_packet.push(array[i]);
                        checksum = u8::overflowing_add(checksum, array[i])
                    }
                )*

                checksum = !checksum;
                checksum = u8::overflowing_add(checksum, 1).0;
                send_packet.push(checksum);

                 for e in  &send_packet{
                }

                const RETURN_PACKET_SIZE:usize = std::mem::size_of::<$struct_name_r>();

                let mut buffer:[u8; RETURN_PACKET_SIZE] = [0; RETURN_PACKET_SIZE];
                unsafe{
                    let bytes_written = write(fd, send_packet); // TODO ERROR CHECK
                    println!("bytes written: {:?}", bytes_written);



                    let bytes_read = read(fd,buffer.len() as i32, buffer.as_mut_ptr());
                    println!("bytes read: {:?}", bytes_read);
                    Some(std::mem::transmute::<[u8; RETURN_PACKET_SIZE],$struct_name_r>(buffer))
                }

            }
        }
    };
}
RESPONSE!(GetPoseR, {
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
    let mut b: [u8; 10];
    let x = std::mem::size_of::<GetPoseR>();
    for element in payload {
        amount += element;
    }
    return !amount + 1;
}

pub fn get_pose(fd: c_int) {
    let ret = GetPoseR::send_command(fd).unwrap();
    let header = ret.header;
    println!("Header: {}\n", header);
}
