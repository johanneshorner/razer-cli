#![feature(os_str_display)]

use anyhow::Context;
use clap::{Parser, Subcommand};
use serde::Serialize;
use std::{ffi::OsStr, fmt::Display};
use udev::Enumerator;

#[derive(Serialize)]
struct Dpi {
    x: u32,
    y: u32,
}

#[derive(Serialize)]
struct QueryResult {
    charge_level: Option<u8>,
    dpi: Option<Dpi>,
}

struct Device(udev::Device);

impl Device {
    fn find_razer_devices() -> anyhow::Result<Vec<Self>> {
        let mut enumerator = Enumerator::new()?;

        enumerator.match_subsystem("hid")?;
        // only filter for the razer vendor id 0x1532
        enumerator.match_property("HID_ID", "*:00001532:*")?;

        Ok(enumerator
            .scan_devices()?
            .filter(|d| d.attribute_value("device_serial").is_some())
            .map(Self)
            .collect())
    }

    fn from_device_serial(device_serial: &str) -> anyhow::Result<Self> {
        let mut enumerator = Enumerator::new()?;

        enumerator.match_subsystem("hid")?;
        enumerator.match_property("HID_ID", "*:00001532:*")?;
        enumerator.match_attribute("device_serial", device_serial)?;

        enumerator
            .scan_devices()?
            .next()
            .with_context(|| format!("device with serial `{device_serial}` not found"))
            .map(Self)
    }

    fn charge_level(&self) -> Option<u8> {
        self.0.attribute_value("charge_level").map(|s| {
            s.to_str()
                .expect("utf8")
                .parse::<u8>()
                .expect("number from 0 to 255")
        })
    }

    fn dpi(&self) -> Option<Dpi> {
        self.0.attribute_value("dpi").map(|s| {
            let (x, y) = s
                .to_str()
                .expect("utf8")
                .split_once(':')
                .expect("dpi in the form of 800:800");
            Dpi {
                x: x.parse::<u32>().expect("number"),
                y: y.parse::<u32>().expect("number"),
            }
        })
    }
}

impl Display for Device {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "Type: {}",
            self.0
                .attribute_value("device_type")
                .expect("attribute device_type to exist")
                .display()
        )?;
        write!(
            f,
            "Serial: {}",
            self.0
                .attribute_value("device_serial")
                .expect("attribute device_type to exist")
                .display()
        )
    }
}

#[derive(Debug, Subcommand)]
enum Command {
    List,
    Query { device_serial: String },
}

#[derive(Debug, Parser)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    match args.command {
        Command::List => {
            let devices = Device::find_razer_devices()?;
            for (i, device) in devices.iter().enumerate() {
                eprintln!("{}:\n{device}\n", i + 1);
            }
        }
        Command::Query { device_serial } => {
            let device = Device::from_device_serial(&device_serial)?;
            let query_result = QueryResult {
                charge_level: device.charge_level(),
                dpi: device.dpi(),
            };
            eprintln!("{}", serde_json::to_string(&query_result)?);
        }
    };

    Ok(())
}
