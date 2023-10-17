use std::ffi::c_int;
use std::os::raw::c_char;
#[allow(warnings)]
// use self::bindings::Device;
mod bindings;
pub mod protocol;

pub unsafe fn serial_open() -> c_int {
    unsafe { return bindings::open_serial_port() };
}

pub unsafe fn read(socket: i32, len: i32, bytes: *mut u8) -> c_int {
    return bindings::device_read(socket, len, bytes);
}
pub unsafe fn write(socket: i32, bytes: Vec<u8>) -> c_int {
    bindings::device_write(socket, bytes.as_ptr().cast_mut(), bytes.len() as i32)
}
