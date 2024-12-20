#![feature(os_str_display)]

use std::fs;
use tracing::{info, trace};
use udev::Enumerator;

fn list_razer_devices() -> anyhow::Result<()> {
    let mut enumerator = Enumerator::new()?;

    // Filter by USB subsystem (assuming Razer devices are USB)
    enumerator.match_subsystem("hid")?;

    // Iterate over devices
    for device in enumerator.scan_devices()? {
        let Some(vendor_id) = device
            .property_value("HID_ID")
            .inspect(|id| trace!("found device with HID-ID `{}`", id.display()))
            .filter(|id| {
                u32::from_str_radix(
                    id.to_str()
                        .expect("hid_id to be valid utf-8")
                        .split(':')
                        .nth(1)
                        .expect("hid_id to be in the form of 0000:00000000:00000000"),
                    16,
                )
                .expect("a base16 number")
                    == 0x1532
            })
        else {
            continue;
        };
        let serial_path = device.syspath().join("device_serial");
        if let Ok(serial) = fs::read_to_string(&serial_path) {
            println!("Found Razer device at {}", device.syspath().display(),);
            println!("vendor_id: {}", vendor_id.display());
            println!(
                "name: {}",
                device.property_value("HID_NAME").unwrap().display()
            );
            println!("serial: {serial}");
        }
    }

    Ok(())
}

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    list_razer_devices()
}
