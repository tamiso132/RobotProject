use robotproject::shared;
use std::{ops::Add, ptr::null, string, u8};
fn main() {
    unsafe {
        let x = std::ffi::CString::new("/dev/tyyUSB0").unwrap().as_ptr();
        shared::open_socketfd(x);
    };
}

extern "C" fn callback(t: *mut u8, size: u32) {}
