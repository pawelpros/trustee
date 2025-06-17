use super::DevicePathParser;
use anyhow::{bail, Result};

pub struct MessagingParser;

impl DevicePathParser for MessagingParser {
    fn parse(&self, sub_type: u8, data: &[u8]) -> Result<String> {
        match sub_type {
            // 0x01 => atapi(data),
            // 0x02 => scsi(data),
            // 0x03 => fibre_channel(data),
            0x15 => fibre_channel_ex(data),
            // 0x04 => i1394(data),
            0x05 => usb(data),
            // 0x06 => i2o(data),
            // 0x09 => infiniband(data),
            // 0x0A => msg_vendor(data),
            0x0B => mac(data),
            // 0x0C => ipv_4(data),
            // 0x0D => ipv_6(data),
            // 0x0E => uart(data),
            // 0x0F => usbclass(data),
            // 0x10 => usbwwid(data),
            // 0x11 => device_logical_unit(data),
            // 0x12 => sata(data),
            // 0x13 => iscsi(data),
            // 0x14 => vlan(data),
            // 0x16 => sas_ex(data),
            // 0x17 => nvme(data),
            // 0x18 => uri(data),
            // 0x19 => ufs(data),
            // 0x1A => sd(data),
            // 0x1B => bluetooth(data),
            // 0x1C => wifi(data),
            // 0x1D => emmc(data),
            // 0x1E => bluetooth_le(data),
            // 0x1F => dns(data),
            // 0x20 => nvdimmservice(data),
            // 0x21 => restservice(data),
            // 0x22 => nvmeo_fnamespace(data),
            _ => {
                bail!("Unknown sub type {}", sub_type)
            }
        }
    }
}

fn fibre_channel_ex(data: &[u8]) -> Result<String> {
    if data.len() < 16 {
        bail!("Fibre channel data is too short");
    }

    let wwn = &data[0..8];
    let lun = &data[8..16];

    Ok(format!("FibreEx(0x{},0x{})", hex::encode(wwn), hex::encode(lun)))
}

fn usb(data: &[u8]) -> Result<String> {
    if data.len() < 2 {
        bail!("USB data is too short");
    }

    let parent_hub_port_num = &data[0];
    let controller_int_number = &data[1];

    Ok(format!("USB({},{})", parent_hub_port_num, controller_int_number))
}

fn mac(data: &[u8]) -> Result<String> {
    if data.len() < 32 {
        bail!("MAC data is too short");
    }

    let mac_address_padded = &data[0..31];
    let binding = hex::encode(mac_address_padded);
    let mac_address = binding.trim_end_matches('0').to_uppercase();
    let if_type = &data[32];

    Ok(format!("Mac({},0x{:02X})", mac_address, if_type))
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(5, "0000", "USB(0,0)")]
    #[case(11, "001320f5fa77000000000000000000000000000000000000000000000000000001", "Mac(001320F5FA77,0x01)"
    )]
    // #[case(11, "0000", "IPv4(192.168.0.100,TCP,Static,192.168.0.1)")]
    #[case(21, "00010203040506070001020304050607", "FibreEx(0x0001020304050607,0x0001020304050607)"
    )]
    fn test_formatter(#[case] sub_type: u8, #[case] data: &str, #[case] expected_result: &str) {
        let parser = MessagingParser;
        let vendor_data = &*hex::decode(data).unwrap();
        let actual = parser.parse(sub_type, vendor_data);
        assert!(actual.is_ok());
        assert_eq!(actual.unwrap(), expected_result);
    }
}