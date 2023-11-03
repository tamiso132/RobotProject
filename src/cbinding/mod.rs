use std::ffi::c_int;
use std::os::raw::c_char;
#[allow(warnings)]
// use self::bindings::Device;
pub mod bindings;

pub unsafe fn serial_open() -> c_int {
    unsafe {
        return bindings::open_serial_port();
    };
}

pub unsafe fn close_port(fd: c_int) {
    bindings::close_socket(fd);
}

pub unsafe fn read(socket: c_int, len: i32, bytes: *mut u8) -> c_int {
    return bindings::device_read(socket, len, bytes);
}
pub unsafe fn write(socket: c_int, bytes: Vec<u8>) -> c_int {
    bindings::device_write(socket, bytes.as_ptr().cast_mut(), bytes.len() as i32)
}
