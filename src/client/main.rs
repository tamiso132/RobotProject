use robotproject::shared;
fn main() {
    unsafe {
        shared::serial_open();
    }
}
