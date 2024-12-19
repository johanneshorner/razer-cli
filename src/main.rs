use std::fs;
use udev::Enumerator;

fn list_razer_devices() -> anyhow::Result<()> {
    let mut enumerator = Enumerator::new()?;

    // Filter by USB subsystem (assuming Razer devices are USB)
    enumerator.match_subsystem("hid")?;

    // Iterate over devices
    for device in enumerator.scan_devices()? {
        // Check for Razer vendor ID (e.g., 0x1532)
        // eprintln!("dev: {device:?}");
        // if let Some(vendor_id) = device.property_value("ID_VENDOR_ID") {
        let serial_path = device.syspath().join("device_serial");

        // Check if the file exists and read it
        if let Ok(serial) = fs::read_to_string(&serial_path) {
            println!(
                "Found Razer device at {}: Serial: {}",
                device.syspath().display(),
                serial.trim()
            );
        } // } else {
        //     println!(
        //         "Razer device at {} has no device_serial file.",
        //         device.syspath().display()
        //     );
        // }
        // }
    }

    Ok(())
}

fn main() -> anyhow::Result<()> {
    list_razer_devices()
}
