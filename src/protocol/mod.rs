use crate::cbinding::{read, write};
use serde::{Deserialize, Serialize};
use std::ffi::c_int;
pub const HEADER: u16 = 0xAA | 0xAA << 8;
pub const QUEUE_FLAG: u8 = 1 << 1;
pub const RW_FLAG: u8 = 1 << 0;

pub struct QueueIndex {}
impl QueueIndex {
    pub fn generate(val: [u8; 8]) -> u64 {
        let num = u64::from_le_bytes(val);
        num
    }
}

#[repr(C, packed)]
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct FloatCustom {
    pub hex_float: [u8; 4],
}

#[repr(C, packed)]
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct IntCustom {
    pub hex_float: [u8; 4],
}

impl IntCustom {
    pub fn new(val:u32) -> Self{
        let hex_float = val.to_le_bytes();
        Self { hex_float }
    }
}


#[repr(C, packed)]
#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub struct Empty;
impl FloatCustom {
    pub fn to_float(&mut self) -> f32 {
        f32::from_le_bytes(self.hex_float.clone())
    }

    pub fn new(f: f32) -> FloatCustom {
        let hex_float = f32::to_le_bytes(f);

        Self { hex_float }
    }
    pub fn increase_by_one(&mut self) {
        self.hex_float = FloatCustom::new(self.to_float() + 1.0).hex_float;
    }

    pub fn test_new(hex_float:[u8; 4]) -> Self{
        Self{
            hex_float
        }
    }
}

macro_rules! response {
    ($struct_name_r:ident, {$($field_s:ident : $ty_s:ty),* }, {$($field_r:ident : $ty_r:ty),* }, $id:expr, $ctrl:expr) => {
        #[repr(C, packed)]
       #[derive(Serialize, Deserialize, Debug, Copy, Clone)]
        pub struct  $struct_name_r {
            pub header: u16,
            pub len: u8,
            pub id: u8,
            pub ctrl: u8,
            $(pub $field_r: $ty_r,)*
            pub checksum:u8,
        }

        impl $struct_name_r{
            pub fn send_immediate_command(fd:c_int, $($field_s: $ty_s),*) -> Option<$struct_name_r>{
                let mut id:u8 = $id;
                let mut ctrl:u8 = 0;
                let mut len:u8 = 2;
                let mut checksum:u8 = 0;
                checksum += $id;
                checksum += $ctrl;


                let mut send_packet:Vec<u8> = vec!();
                $(
                    len += std::mem::size_of::<$ty_s>() as u8;
                )*

                let mut header1_list = bincode::serialize(&HEADER).unwrap();
                let mut len_list = bincode::serialize(&len).unwrap();
                let mut id_list = bincode::serialize(&id).unwrap();
                let mut ctrl_list = bincode::serialize(&ctrl).unwrap();

                send_packet.append(&mut header1_list);
                send_packet.append(&mut len_list);
                send_packet.append(&mut id_list);
                send_packet.append(&mut ctrl_list);
                $(
                    let mut field = bincode::serialize(&$field_s).unwrap();
                    send_packet.append(&mut field);
                    for f in &field{
                        checksum = u8::overflowing_add(checksum, f).0;
                    }
                )*

                checksum = !checksum;
                checksum = u8::overflowing_add(checksum, 1).0;
                send_packet.push(checksum);

                for e in &send_packet{
                }

                const RETURN_PACKET_SIZE:usize = std::mem::size_of::<$struct_name_r>();

                let mut buffer:[u8; 256] = [0; 256];
                unsafe{
                    let bytes_written = write(fd, send_packet); // TODO ERROR CHECK
                    let bytes_read = read(fd,256, buffer.as_mut_ptr());

                    for i in 0..bytes_read{
                    }
                    let work = bincode::deserialize::<$struct_name_r>(&buffer).unwrap();
                    Some(work)
                }

            }
        }
    };
}
///
/// This is if the specific command has both a set and get
///
/// Parameters
/// (Structure Name, Fields, ID)
macro_rules! response2 {

    ($struct_name:ident, {$($field:ident : $ty:ty),* }, $id:expr) => {
       #[repr(C, packed)]
       #[derive(Serialize, Deserialize, Debug, Clone, Copy)]
        pub struct  $struct_name {
            pub header: u16,
            pub len: u8,
            pub id: u8,
            pub ctrl: u8,
            $(pub $field: $ty,)*
            pub checksum:u8,
        }

        impl $struct_name{
            pub fn send_immediate_command(fd:c_int, $($field: &$ty),*) -> Option<u64>{
                let mut ret =  Self::send_packet(fd, 0, $(&$field),*);
            //    while ret.1 <= 0 {
            //         unsafe{
            //         ret = Self::send_packet(fd, 0, $(&$field),*);
            //         std::thread::sleep(std::time::Duration::from_millis(200));
            //     }
             //  }
               ret.0
            }
            pub fn send_queue_command(fd: c_int, $($field: &$ty),*) -> Option<u64>{

               let mut ret =  Self::send_packet(fd, 1, $(&$field),*);
            //    while ret.1 <= 0 {
            //     unsafe{
            //         ret = Self::send_packet(fd, 1, $(&$field),*);
            //         std::thread::sleep(std::time::Duration::from_millis(200));
            //     }
            //    }
               ret.0

            }
            pub fn send_get_command(fd:c_int) -> Option<$struct_name>{
                 let mut id:u8 = $id;
                let mut ctrl:u8 = 0;
                let mut len:u8 = 2;
                let mut checksum:u8 = 0;
                checksum += $id;
                checksum += ctrl;


                let mut send_packet:Vec<u8> = vec!();

                let mut header1_list = bincode::serialize(&HEADER).unwrap();
                let mut len_list = bincode::serialize(&len).unwrap();
                let mut id_list = bincode::serialize(&id).unwrap();
                let mut ctrl_list = bincode::serialize(&ctrl).unwrap();

                send_packet.append(&mut header1_list);
                send_packet.append(&mut len_list);
                send_packet.append(&mut id_list);
                send_packet.append(&mut ctrl_list);

                checksum = !checksum;
                checksum = u8::overflowing_add(checksum, 1).0;
                send_packet.push(checksum);



                const RETURN_PACKET_SIZE:usize = std::mem::size_of::<$struct_name>();

                for e in &send_packet{
                }

                let mut buffer:[u8; 256] = [0; 256];
                unsafe{
                    let bytes_written = write(fd, send_packet); // TODO ERROR CHECK
                    let bytes_read = read(fd,256, buffer.as_mut_ptr());
                    let work = bincode::deserialize::<$struct_name>(&buffer).unwrap();
                    Some(work)
                }

                // let mut header = bincode::serialize(&HEADER).unwrap();
                // let id:u8 = $id;
                // let ctrl:u8 = 0;
                // let len:u8 = 2;
                // let mut checksum:u8 = 0;
                // checksum = checksum.overflowing_add(id).0;
                // checksum = checksum.overflowing_add(ctrl).0;

                // checksum = !checksum;
                // checksum = checksum.overflowing_add(1).0;

                // let mut send_packet:Vec<u8> = vec!();

                // send_packet.append(&mut header);
                // send_packet.push(len);
                // send_packet.push(id);
                // send_packet.push(ctrl);
                // send_packet.push(checksum);

                // const RETURN_PACKET_SIZE:usize = std::mem::size_of::<$struct_name>();

                // let mut buffer:[u8; 256] = [0; 256];
                // for e in &send_packet{
                //   //  println!("{}", e);
                // }
                // unsafe{
                //     let bytes_written = write(fd, send_packet);
                //     let bytes_read = read(fd,256, buffer.as_mut_ptr());
                //     println!("bytes read: {}", bytes_read);
                //     // for index in 0..bytes_read-1{
                //     //     print!("{:#02x} ", buffer[index as usize]);
                //     // }
                //     let ret = bincode::deserialize::<$struct_name>(&buffer).unwrap();
                //     Some(ret)
                // }
            }

            fn send_packet(fd:c_int, queue: u8, $($field: &$ty),*) -> (Option<u64>, i32){
             let mut header = bincode::serialize(&HEADER).unwrap();
                let id:u8 = $id;  // 0, 1, 2, 3, 4,
                let mut ctrl = RW_FLAG;
                if queue == 1{
                    ctrl |= QUEUE_FLAG;
                }
                let mut len:u8 = 2;
                let mut checksum:u8 = 0;
                checksum = checksum.overflowing_add(id).0;
                checksum = checksum.overflowing_add(ctrl).0;

                let mut s_packet:Vec<u8> = vec!();
                let mut data_vec:Vec<u8> = vec!();


                $(
                    let mut field = bincode::serialize(&$field).unwrap();
                    len += field.len() as u8;
                    println!("Len: {}", field.len());
                    for f in &field{
                        checksum = u8::overflowing_add(*f, checksum).0;

                    }
                    data_vec.append(&mut field);

                )*


               // println!("Checksum {}", checksum);
              //  println!("Checksum total {:#02b}", checksum);
                checksum = !checksum;
              //  println!("Checksum inverse {:#02b}", checksum);
                checksum = u8::overflowing_add(checksum, 1).0;
               // println!("Checksum Done {:#02b}", checksum);
              //  println!("Len: {}", len);

                s_packet.append(&mut header);
                s_packet.push(len);
                s_packet.push(id);
                s_packet.push(ctrl);
                s_packet.append(&mut data_vec);
                s_packet.push(checksum); // AA AA 02 3E 00 F4


                 for e in &s_packet{
                    //println!("{}", e);
                }
                //  println!("New Start");

                unsafe{
                    let bytes_written = write(fd, s_packet);
                    std::thread::sleep(std::time::Duration::from_millis(100));
                    let mut buffer:[u8; 256] = [0; 256];
                    let bytes_read = read(fd, buffer.len() as i32, buffer.as_mut_ptr());

                    println!("bytes read: {}", bytes_read);

                    for i in 0..bytes_read{
                        println!("{}", &buffer[i as usize]);
                    }
                    if queue == 1 && bytes_read != 0{
                        let mut queue_buffer:[u8;8] = [0, 0, 0, 0, 0, 0, 0, 0];
                        queue_buffer.copy_from_slice(&buffer[5..13]);
                        return (Some(QueueIndex::generate(queue_buffer)), bytes_read);
                    }



                    (None, bytes_read)
                }
            }
        }

    };
}

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
    let mut ctrl: u8 = 1 >> 7 | 1 >> 6;
    return !amount + 1;
}

response!(GetPoseR, {},
{ 
    x: FloatCustom,
    y: FloatCustom,
    z: FloatCustom,
    r: FloatCustom,
    joint_angle: [FloatCustom; 4]
}, 10, 0);

response2!(SuctionCup, {is_ctrl:u8, is_sucked:u8}, 62);
response2!(JogJointParam, {velocity:[FloatCustom; 4], acceleration:[FloatCustom; 4]}, 70);
response2!(JogCoordinateParam, {velocity:[FloatCustom; 4], acceleration:[FloatCustom; 4]}, 71);
response2!(EMotor, {index: u8, enabled:u8, speed: IntCustom}, 135);
pub mod ptp {

    use crate::cbinding::{read, write};
    use crate::protocol::*;
    use bincode::config::BigEndian;
    use serde::{Deserialize, Serialize};
    use std::ffi::c_float;
    use std::ffi::c_int;
    use std::mem::size_of;
    use std::mem::transmute;
    use std::u8;

    use super::FloatCustom;

    response2!(Joint, {velocity:[FloatCustom; 4], acceleration:[FloatCustom;4]}, 80);
    response2!(Coordinate, {xyz_velocity: FloatCustom, r_velocity: FloatCustom, xyz_acceleration:FloatCustom, r_acceleration:FloatCustom}, 81);
    response2!(Jump, {jump_height: FloatCustom, z_limit:FloatCustom}, 82);
    response2!(Common, {velocity_ratio: FloatCustom, acceleration_ratio:FloatCustom}, 83);
    response2!(Cmd, {ptp_mode: PTPMode, x:FloatCustom, y :FloatCustom, z:FloatCustom, r:FloatCustom }, 84);

    #[repr(u8)]
    #[derive(Serialize, Deserialize, Debug, Copy, Clone)]
    #[serde(into = "u8")]
    pub enum PTPMode {
        JumpXYZ = 0, // JUMP mode, (x,y,z,r) is the target point in Cartesian coordinate system
        MovjXYZ,     // MOVJ mode, (x,y,z,r) is the target point in Cartesian coordinate system
        MovlXYZ,     //MOVL mode, (x,y,z,r) is the target point in Cartesian coordinate system
        JumpANGLE,   // JUMP mode, (x,y,z,r) is the target point in Joint coordinate system
        MovjANGLE,   // MOVJ mode, (x,y,z,r) is the target point in Joint coordinate system
        MovlANGLE,   // MOVL mode, (x,y,z,r) is the target point in Joint coordinate system
        MovjINC,     // MOVJ mode, (x,y,z,r) is the angle increment in Joint coordinate system
        MovlINC, // MOVL mode, (x,y,z,r) is the Cartesian coordinate increment in Joint coordinate system
        MovjXYZINC, // MOVJ mode, (x,y,z,r) is the Cartesian coordinate increment in Cartesian coordinate
    }

    impl From<PTPMode> for u8 {
        fn from(value: PTPMode) -> u8 {
            value as u8
        }
    }
}

pub mod queue {
    use crate::cbinding::{read, write};
    use crate::protocol::*;
    use bincode::config::BigEndian;
    use serde::{Deserialize, Serialize};
    use std::ffi::c_float;
    use std::ffi::c_int;
    use std::mem::size_of;
    use std::mem::transmute;

    use super::FloatCustom;

    response2!(StartExec, {}, 240);
    response2!(StopExec, {}, 241);
    response2!(ClearExec, {}, 245);
    response2!(CurrentIndex, { current_index: u64 }, 246);
    response2!(StopForceExec, {}, 242);
}

pub mod homing {
    use crate::cbinding::{read, write};
    use crate::protocol::*;
    use bincode::config::BigEndian;
    use serde::{Deserialize, Serialize};
    use std::ffi::c_float;
    use std::ffi::c_int;
    use std::mem::size_of;
    use std::mem::transmute;

    use super::FloatCustom;

    response2!(Param, {x:FloatCustom, y: FloatCustom, z:FloatCustom, r:FloatCustom}, 30);
    response2!(Cmd, { temp: u32 }, 31);
}
