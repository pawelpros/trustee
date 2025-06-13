use super::DevicePathParser;
use anyhow::{bail, Result};

pub struct AcpiParser;

impl DevicePathParser for AcpiParser {
    fn parse(&self, sub_type: u8, data: &[u8]) -> Result<String> {
        match sub_type {
            0x01 => acpi(data),
            // 0x02 => acpiExpanded(data),
            // 0x03 => acpiAdr(data),
            _ => {
                bail!("Unknown sub type {}", sub_type)
            }
        }
    }
}

fn acpi(data: &[u8]) -> Result<String> {
    if data.len() < size_of::<u64>() {
        bail!("Data length must be at least 8 bytes");
    }

    let hid = u32::from_le_bytes(data[0..4].try_into()?);
    let uid = u32::from_le_bytes(data[4..8].try_into()?);

    let vendor = hid & 0xFFFF;

    let vendor1 = ((vendor >> 10) & 0x1F) as u8 + b'@';
    let vendor2 = ((vendor >> 5) & 0x1F) as u8 + b'@';
    let vendor3 = (vendor & 0x1F) as u8 + b'@';

    let device = hid >> 16;

    let hid_formatted = format!(
        "{}{}{}{:04X}",
        vendor1 as char, vendor2 as char, vendor3 as char, device
    );

    Ok(format!("ACPI({},{})", hid_formatted, uid))
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(1, "d041030a00000000", "ACPI(PNP0A03,0)")]
    #[case(1, "d041030a02000000", "ACPI(PNP0A03,2)")]
    fn test_formatter(#[case] sub_type: u8, #[case] data: &str, #[case] expected_result: &str) {
        let parser = AcpiParser;
        let vendor_data = &*hex::decode(data).unwrap();
        let actual = parser.parse(sub_type, vendor_data);
        assert!(actual.is_ok());
        assert_eq!(actual.unwrap(), expected_result);
    }
}
