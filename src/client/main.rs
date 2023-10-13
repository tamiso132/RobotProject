use robotproject::shared;
fn main() {
    unsafe {
        let fd = shared::serial_open();
    }
}
