#[allow(warnings)]
// use self::bindings::Device;
mod bindings;

pub fn device_connect() {
    unsafe {
        bindings::u_device_scan();
    }
}
