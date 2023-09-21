// See the "macOS permissions note" in README.md before running this on macOS
// Big Sur or later.

use std::arch::x86_64::_mm_div_pd;
use std::error::Error;
use std::time::Duration;
use tokio::time;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    Ok(())
}
