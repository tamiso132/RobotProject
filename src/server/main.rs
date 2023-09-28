use robotproject::shared;
use std::{ops::Add, ptr::null, string, u8};
fn main() {
    unsafe {
        let x = std::ffi::CString::new("/dev/ttyUSB0").unwrap();
        let fd = shared::open_socketfd(x.as_ptr());
        let mut y = "y".as_bytes().to_vec();
        shared::write(fd, y);
        loop {}
    };
}

extern "C" fn callback(t: *mut u8, size: u32) {}
