#[allow(warnings)]
// use self::bindings::Device;
mod bindings;

pub fn device_connect() {
    unsafe {
        let socket_fd = bindings::u_init_server();
    }
}
