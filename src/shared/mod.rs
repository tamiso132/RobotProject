#[allow(warnings)]
// use self::bindings::Device;
mod bindings;

pub fn device_connect() {
    unsafe {
        let socket_fd = bindings::u_init_server();
    }
}

pub fn connect_server() {
    unsafe {
        let s = "192.168.0.107".as_bytes();

        let socket_fd = bindings::u_device_connect(s.as_ptr());
    }
}
