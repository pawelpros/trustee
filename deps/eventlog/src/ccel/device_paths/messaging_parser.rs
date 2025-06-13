use super::{DevicePathParser};
use anyhow::{bail, Result};

pub struct MessagingParser;

impl DevicePathParser for MessagingParser {
    fn parse(&self, sub_type: u8, _data: &[u8]) -> Result<String> {
        match sub_type {
            // 0x01) => Atapi,
            // 0x02) => Scsi,
            // 0x03) => FibreChannel,
            // 0x15) => FibreChannelEx,
            // 0x04) => I1394,
            // 0x05) => Usb,
            // 0x06) => I2O,
            // 0x09) => Infiniband,
            // 0x0A) => MsgVendor,
            // 0x0B) => Mac,
            // 0x0C) => IPv4,
            // 0x0D) => IPv6,
            // 0x0E) => UART,
            // 0x0F) => USBClass,
            // 0x10) => USBWWID,
            // 0x11) => DeviceLogicalUnit,
            // 0x12) => SATA,
            // 0x13) => ISCSI,
            // 0x14) => VLAN,
            // 0x16) => SasEx,
            // 0x17) => NVMe,
            // 0x18) => Uri,
            // 0x19) => Ufs,
            // 0x1A) => Sd,
            // 0x1B) => Bluetooth,
            // 0x1C) => Wifi,
            // 0x1D) => Emmc,
            // 0x1E) => BluetoothLE,
            // 0x1F) => Dns,
            // 0x20) => NVDIMMService,
            // 0x21) => RESTService,
            // 0x22) => NVMeoFNamespace,
            _ => { bail!("Unknown sub type {}", sub_type) }
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use rstest::rstest;
//
//     #[rstest]
//     #[case(
//         "73616d706c6520746573742064617461",
//         EventDetails::from_string(String::from("sample test data"))
//     )]
//     #[case(
//         "43616c6c696e6720454649204170706c69636174696f6e2066726f6d20426f6f74204f7074696f6e",
//         EventDetails::from_string(String::from("Calling EFI Application from Boot Option"))
//     )]
//     #[case::blank("", EventDetails::from_string(String::from("")))]
//     #[case::not_utf_part("0F", EventDetails::from_string(String::from("\u{f}")))]
//     fn test_simple_parser(#[case] test_data: &str, #[case] expected_result: EventDetails) {
//         let parser = EvSimpleParser;
//         let actual_result = parser.parse(hex::decode(test_data).unwrap());
//
//         assert!(actual_result.is_ok());
//         assert_eq!(actual_result.unwrap(), expected_result);
//     }
// }
