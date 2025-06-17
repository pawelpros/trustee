use super::DevicePathParser;
use anyhow::{bail, Result};

pub struct HardwareParser;

impl DevicePathParser for HardwareParser {
    fn parse(&self, sub_type: u8, data: &[u8]) -> Result<String> {
        match sub_type {
            0x01 => pci(data),
            // 0x02 => pcCard(data),
            // 0x03 => memoryMapped(data),
            // 0x04 => vendor(data),
            // 0x05 => controller(data),
            // 0x06 => bmc(data),
            _ => {
                bail!("Unknown sub type {}", sub_type)
            }
        }
    }
}

fn pci(data: &[u8]) -> Result<String> {
    if data.len() < 2 {
        bail!("PCI data is too short");
    }
    let func_num = data[0];
    let device_num = data[1];
    Ok(format!("Pci({},{})", func_num, device_num))
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(1, "0001", "Pci(0,1)")]
    #[case(1, "0000", "Pci(0,0)")]
    #[case(1, "0004", "Pci(0,4)")]
    fn test_formatter(#[case] sub_type: u8, #[case] data: &str, #[case] expected_result: &str) {
        let parser = HardwareParser;
        let vendor_data = &*hex::decode(data).unwrap();
        let actual = parser.parse(sub_type, vendor_data);
        assert!(actual.is_ok());
        assert_eq!(actual.unwrap(), expected_result);
    }
}
