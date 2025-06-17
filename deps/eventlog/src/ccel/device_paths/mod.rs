mod acpi_parser;
mod bios_boot_spec_parser;
mod hardware_parser;
mod media_parser;
mod messaging_parser;
mod unknown_device_path_parser;

pub(crate) use crate::ccel::device_paths::acpi_parser::AcpiParser;
pub(crate) use crate::ccel::device_paths::bios_boot_spec_parser::BiosBootSpecParser;
pub(crate) use crate::ccel::device_paths::hardware_parser::HardwareParser;
pub(crate) use crate::ccel::device_paths::media_parser::MediaParser;
pub(crate) use crate::ccel::device_paths::messaging_parser::MessagingParser;
pub(crate) use crate::ccel::device_paths::unknown_device_path_parser::UnknownDevicePathParser;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum DevicePathType {
    Hardware = 0x01,
    Acpi = 0x02,
    Messaging = 0x03,
    Media = 0x04,
    BiosBootSpec = 0x05,
    End = 0x7F,
    Unknown(u8),
}

impl From<u8> for DevicePathType {
    fn from(value: u8) -> Self {
        match value {
            0x01 => Self::Hardware,
            0x02 => Self::Acpi,
            0x03 => Self::Messaging,
            0x04 => Self::Media,
            0x05 => Self::BiosBootSpec,
            0x7F => Self::End,
            other => Self::Unknown(other),
        }
    }
}

impl DevicePathType {
    pub(crate) fn get_parser(&self) -> Box<dyn DevicePathParser> {
        match self {
            Self::Hardware => Box::new(HardwareParser),
            Self::Acpi => Box::new(AcpiParser),
            Self::Messaging => Box::new(MessagingParser),
            Self::Media => Box::new(MediaParser),
            Self::BiosBootSpec => Box::new(BiosBootSpecParser),
            _ => Box::new(UnknownDevicePathParser),
        }
    }
}

pub trait DevicePathParser: Sync + Send {
    fn parse(&self, sub_type: u8, data: &[u8]) -> anyhow::Result<String>;
}
