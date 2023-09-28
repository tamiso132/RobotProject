use std::os::raw::c_char;

#[allow(warnings)]
// use self::bindings::Device;
mod bindings;

pub unsafe fn serial_open() -> std::ffi::c_int {
    bindings::open_serial_port()
}

pub fn open_socketfd(name: *const std::ffi::c_char) -> std::ffi::c_int {
    unsafe { return bindings::file_open_and_get_descriptor(name) };
}

pub unsafe fn read(socket: i32, len: i32, bytes: *mut u8) {
    bindings::device_read(socket, len, bytes);
}
pub unsafe fn write(socket: i32, bytes: Vec<u8>) {
    bindings::device_write(socket, bytes.as_ptr().cast_mut(), bytes.len() as i32)
}
