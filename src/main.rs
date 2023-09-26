use std::{ops::Add, ptr::null, string, u8};

use robotproject::bindings;

fn main() {
    unsafe {
        let devices = bindings::device_scan();
        if devices.is_null() || (*devices).size == 0 {
            println!("no devices found");
            return;
        }

        let mut device = *((*devices).head);

        for _ in 0..((*devices).size - 1) {}
    };
}

extern "C" fn callback(t: *mut u8, size: u32) {}
