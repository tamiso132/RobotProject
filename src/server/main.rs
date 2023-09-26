use robotproject::shared;
use std::{ops::Add, ptr::null, string, u8};
fn main() {
    unsafe {
        shared::device_connect();
    };
}

extern "C" fn callback(t: *mut u8, size: u32) {}
