use anyhow::{anyhow, Result};

pub fn check_for_guest_device() -> Result<()> {
    match std::fs::File::open("/dev/tdx_guest") {
        Ok(_) => {
            println!("/dev/tdx_guest device is present");
        }
        Err(e) => {
            return Err(anyhow!("failure checking /dev/tdx_guest: {}", e));
        }
    }

    Ok(())
}
