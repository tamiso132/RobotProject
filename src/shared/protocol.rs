use super::{read, write};
use serde::{Deserialize, Serialize};
use std::ffi::c_float;
use std::ffi::c_int;
use std::mem::size_of;
use std::mem::transmute;
pub const HEADER: u16 = 0xAA | 0xAA << 8;

#[repr(C, packed)]
#[derive(Serialize, Deserialize, Debug)]
pub struct FloatCustom {
    hex_float: [u8; 4],
}
impl FloatCustom {
    pub fn to_float(&mut self) -> f32 {
        for e in &self.hex_float {
            print!("hex: {:#02x}\n", e);
        }
        let hex_big: u32 = (self.hex_float[3] as u32)
            | (self.hex_float[2] as u32) << 8
            | (self.hex_float[1] as u32) << 16
            | (self.hex_float[3] as u32) << 24;

        println!("{:#02x}, ", hex_big);
        f32::from_bits(hex_big)
    }
}

macro_rules! RESPONSE {
    ($struct_name_r:ident, {$($field_r:ident : $ty_r:ty),* }, {$($field_s:ident : $ty_s:ty),* }, $id:expr, $ctrl:expr) => {
        #[repr(C)]
       #[derive(Serialize, Deserialize, Debug)]
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
                let mut id:u8 = $id;
                let mut ctrl:u8 = $ctrl;
                let mut len:u8 = 2;
                let mut checksum:u8 = 0;
                checksum += $id;
                checksum += $ctrl;


                let mut send_packet:Vec<u8> = vec!();
                $(
                    len += std::mem::size_of::<$ty_s>();
                )*

                let mut header1_list = bincode::serialize(&HEADER).unwrap();
                for e in &header1_list{
                    println!("Header: {}", e);
                }
                let mut len_list = bincode::serialize(&len).unwrap();
                let mut id_list = bincode::serialize(&id).unwrap();
                let mut ctrl_list = bincode::serialize(&ctrl).unwrap();

                send_packet.append(&mut header1_list);
                send_packet.append(&mut len_list);
                send_packet.append(&mut id_list);
                send_packet.append(&mut ctrl_list);
                $(
                    let mut field = bincode::serialize($field_s).unwrap();
                    send_packet.append(&mut field);
                )*

                checksum = !checksum;
                checksum = u8::overflowing_add(checksum, 1).0;
                send_packet.push(checksum);

                 for e in  &send_packet{
                    println!("{}", e);
                }

                const RETURN_PACKET_SIZE:usize = std::mem::size_of::<$struct_name_r>();

                let mut buffer:[u8; RETURN_PACKET_SIZE] = [0; RETURN_PACKET_SIZE];
                unsafe{
                    let bytes_written = write(fd, send_packet); // TODO ERROR CHECK
                    let bytes_read = read(fd,buffer.len() as i32, buffer.as_mut_ptr());
                    println!("{:#02x}", &buffer[5]);
                    println!("{:#02x}", &buffer[6]);
                    println!("{:#02x}", &buffer[7]);
                    println!("{:#02x}", &buffer[8]); // problem with deserializing
                    let work = bincode::deserialize::<$struct_name_r>(&buffer).unwrap();
                    Some(work)
                    //Some(std::mem::transmute::<[u8; RETURN_PACKET_SIZE],$struct_name_r>(buffer))
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
}
