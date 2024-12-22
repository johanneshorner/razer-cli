use anyhow::{Context, bail};
use clap::{Parser, Subcommand, ValueEnum};
use serde::Serialize;
use std::str::FromStr;
use udev::Enumerator;

#[derive(Debug, Clone, ValueEnum)]
enum PollRate {
    #[value(name = "125")]
    OneTwentyFive,
    #[value(name = "500")]
    FiveHundred,
    #[value(name = "1000")]
    OneThousand,
}

impl PollRate {
    fn as_u32(&self) -> u32 {
        match self {
            PollRate::OneTwentyFive => 125,
            PollRate::FiveHundred => 500,
            PollRate::OneThousand => 1000,
        }
    }
}

impl FromStr for PollRate {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "125" => PollRate::OneTwentyFive,
            "500" => PollRate::FiveHundred,
            "1000" => PollRate::OneThousand,
            _ => bail!("poll-rate needs to be 125, 500 or 1000"),
        })
    }
}

#[derive(Debug, Serialize, clap::Args)]
struct Dpi {
    x: u16,
    y: u16,
}

#[derive(Serialize)]
struct All {
    charge_level: Option<u8>,
    dpi: Option<Dpi>,
    poll_rate: Option<u32>,
}

#[derive(Debug, Clone, ValueEnum)]
enum GetAttribute {
    ChargeLevel,
    Dpi,
    PollRate,
}

impl GetAttribute {
    fn all(device: &Device) -> All {
        All {
            charge_level: device.charge_level(),
            dpi: device.dpi(),
            poll_rate: device.poll_rate().map(|p| p.as_u32()),
        }
    }
}

#[derive(Debug, Subcommand)]
enum SetAttribute {
    Dpi(Dpi),
    PollRate { poll_rate: PollRate },
}

#[derive(Serialize)]
struct DeviceInformation {
    #[serde(rename(serialize = "type"))]
    ty: String,
    serial: String,
    firmware_version: String,
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

    fn ty(&self) -> &str {
        self.0
            .attribute_value("device_type")
            .expect("attribute device_type to exist")
            .to_str()
            .expect("utf8")
    }

    fn serial(&self) -> &str {
        self.0
            .attribute_value("device_serial")
            .expect("attribute device_serial to exist")
            .to_str()
            .expect("utf8")
    }

    fn firmware_version(&self) -> &str {
        self.0
            .attribute_value("firmware_version")
            .expect("attribute firmware_version to exist")
            .to_str()
            .expect("utf8")
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
                x: x.parse::<_>().expect("number"),
                y: y.parse::<_>().expect("number"),
            }
        })
    }

    fn poll_rate(&self) -> Option<PollRate> {
        self.0
            .attribute_value("poll_rate")
            .map(|s| s.to_str().expect("utf8").parse::<PollRate>().unwrap())
    }

    fn set_dpi(&mut self, dpi: &Dpi) -> anyhow::Result<()> {
        let bytes = [dpi.x.to_be_bytes(), dpi.y.to_be_bytes()].concat();
        // not entirely sure why but udev::Device::set_attribute_value fails with `Invalid Argument` here
        // probably has something to do with NUL byte or encoding
        std::fs::write(self.0.syspath().join("dpi"), &bytes)?;
        Ok(())
    }

    fn set_poll_rate(&mut self, poll_rate: PollRate) -> anyhow::Result<()> {
        self.0
            .set_attribute_value("poll_rate", poll_rate.as_u32().to_string())?;
        Ok(())
    }
}

#[derive(Debug, Subcommand)]
enum Command {
    List,
    Get {
        device_serial: String,
        attribute: Option<GetAttribute>,
    },
    Set {
        device_serial: String,
        #[command(subcommand)]
        attribute: SetAttribute,
    },
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
            let device_information: Vec<DeviceInformation> = Device::find_razer_devices()?
                .iter()
                .map(|d| DeviceInformation {
                    ty: d.ty().into(),
                    serial: d.serial().into(),
                    firmware_version: d.firmware_version().into(),
                })
                .collect();
            println!("{}", serde_json::to_string(&device_information)?);
        }
        Command::Get {
            device_serial,
            attribute,
        } => {
            let device = Device::from_device_serial(&device_serial)?;
            if let Some(attribute) = attribute {
                let json = match attribute {
                    GetAttribute::ChargeLevel => serde_json::to_string(&device.charge_level())?,
                    GetAttribute::Dpi => serde_json::to_string(&device.dpi())?,
                    GetAttribute::PollRate => {
                        serde_json::to_string(&device.poll_rate().map(|p| p.as_u32()))?
                    }
                };
                println!("{json:?}");
            } else {
                println!("{}", serde_json::to_string(&GetAttribute::all(&device))?);
            }
        }
        Command::Set {
            device_serial,
            attribute,
        } => {
            let mut device = Device::from_device_serial(&device_serial)?;
            match attribute {
                SetAttribute::Dpi(dpi) => device.set_dpi(&dpi)?,
                SetAttribute::PollRate { poll_rate } => device.set_poll_rate(poll_rate)?,
            }
        }
    };

    Ok(())
}
