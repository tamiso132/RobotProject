use robotproject::shared;
fn main() {
    unsafe {
        let fd = shared::serial_open();
        let mut bytes = vec![];
        shared::read(fd, 1, bytes.as_mut_ptr());
    }
}
