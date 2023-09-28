use std::os::raw::c_char;

#[allow(warnings)]
// use self::bindings::Device;
mod bindings;

pub unsafe fn serial_open() -> std::ffi::c_int {
    bindings::open_serial_port()
}

pub fn open_socketfd(name: *const std::ffi::c_char) {
    unsafe { bindings::file_open_and_get_descriptor(name) };
}
