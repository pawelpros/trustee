// impl DevicePathFormatter for DevicePathSubType {
//     fn format(&self, vendor_data: &[u8]) -> Result<String, FormatError> {
//         use DevicePathSubType::*;
//         match self {
//             // HardDrive => {
//             //     let partition_number = u32::from_le_bytes(vendor_data[0..4].try_into().unwrap());
//             //     let partition_start_lba = u64::from_le_bytes(vendor_data[4..12].try_into().unwrap());
//             //     let partition_size_lba = u64::from_le_bytes(vendor_data[12..20].try_into().unwrap());
//             //     let partition_signature = &vendor_data[20..36];
//             //
//             //     let partition_format = &vendor_data[36];
//             //     let signature_type = &vendor_data[37];
//             //     let part_format = "GPT";
//             //     let guid = format!(
//             //         "{}-{}-{}-{}-{}",
//             //         hex::encode(&partition_signature[0..4]), //TODO SWAP BYTES
//             //         hex::encode(&partition_signature[4..6]), //TODO SWAP BYTES
//             //         hex::encode(&partition_signature[6..8]), //TODO SWAP BYTES
//             //         hex::encode(&partition_signature[8..10]),
//             //         hex::encode(&partition_signature[10..16])
//             //     ).to_uppercase();
//             //     Ok(format!("HD({},{},{},{:#x},{:#x})",partition_number, part_format,guid, partition_start_lba,partition_size_lba))
//             // }
//             // Usb => {
//             //     if vendor_data.len() < 2 {
//             //         return Err(FormatError::InvalidLength);
//             //     }
//             //     let port = vendor_data[0];
//             //     let endpoint = vendor_data[1];
//             //     Ok(format!("Usb({:#04x},{:#04x})", port, endpoint))
//             // }
//             // MAC => {
//             //     if vendor_data.len() < 6 {
//             //         return Err(FormatError::InvalidLength);
//             //     }
//             //     let mac = &vendor_data[..6];
//             //     Ok(format!(
//             //         "MAC({:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X})",
//             //         mac[0], mac[1], mac[2], mac[3], mac[4], mac[5]
//             //     ))
//             // }
//             // Vendor => {
//             //     if vendor_data.len() < 16 {
//             //         return Err(FormatError::InvalidLength);
//             //     }
//             //     let guid = &vendor_data[..16];
//             //     Ok(format!("VenHw({:02X?})", guid)) // Could parse UUID properly
//             // }
//         }
//     }
// }
//
// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::ccel::device_path_enum::DevicePathSubType;
//     use crate::ccel::device_path_enum::DevicePathType;
//     use rstest::rstest;
//
//     // TODO USE IT https://github.com/ecks/uefi/tree/main/efi/efitypes
//
//     #[rstest]
//     #[case(1, 1, "0001", "Pci(0,1)")]
//     #[case(1, 1, "0000", "Pci(0,0)")]
//     #[case(1, 1, "0004", "Pci(0,4)")]
//     #[case(2, 1, "d041030a00000000", "ACPI(PNP0A03,0)")]
//     #[case(3, 23, "010000000000000000000000", "Path(3,23,010000000000000000000000)")]
//     // #[case(4, 1, "0f000000002800000000000000500300000000005e27f1007553d5439eb5de2add4c99320202", "HD(1,GPT,CB96A8E9-ADDC-4BE7-B68E-69B6D1EA59CB,0x800,0x100000")] // TODO INVALID
//     // #[case(4, 1, "0100000000080000000000000000100000000000e9a896cbdcade74bb68e69b6d1ea59cb0202", "HD(1,GPT,CB96A8E9-ADDC-4BE7-B68E-69B6D1EA59CB,0x800,0x100000)")]
//     // #[case(4, 3, "f8d1c555cd04b5468a20e56cbb3052d0", "VenMedia(55C5D1F8-04CD-46B5-8A20-E56CBB3052D0,)")] // TODO VALID
//     // #[case(4, 3, "72f728144ab61e44b8c39ebdd7f893c7", "VenMedia(55C5D1F8-04CD-46B5-8A20-E56CBB3052D0,)")] // TODO INVALID
//     #[case(4, 4, "6b00650072006e0065006c000000", "File(kernel)")]
//     #[case(4, 4, "5c004500460049005c0042004f004f0054005c0042004f004f0054005800360034002e004500460049000000", "File(\\EFI\\BOOT\\BOOTX64.EFI)")]
//     #[case(4, 4, "5c004500460049005c0042004f004f0054005c00660062007800360034002e006500660069000000", "File(\\EFI\\BOOT\\fbx64.efi)")]
//     #[case(4, 4, "5c004500460049005c007500620075006e00740075005c007300680069006d007800360034002e006500660069000000", "File(\\EFI\\ubuntu\\shimx64.efi)")]
//     #[case(4, 4, "5c004500460049005c007500620075006e00740075005c0067007200750062007800360034002e006500660069000000", "File(\\EFI\\ubuntu\\grubx64.efi)")]
//     fn test_formatter(
//         #[case] efi_type: u8,
//         #[case] efi_sub_type: u8,
//         #[case] data: &str,
//         #[case] expected_result: &str,
//     ) {
//         let dev_type = DevicePathType::from(efi_type);
//         let dev_sub_type = DevicePathSubType::from(dev_type, efi_sub_type);
//         let vedor_data = &*hex::decode(data).unwrap();
//         let actual = match dev_sub_type.format(vedor_data) {
//             Ok(text) => text,
//             Err(_) => {
//                 format!(
//                     "Path({},{},{})",
//                     efi_type,
//                     efi_sub_type,
//                     hex::encode(vedor_data)
//                 )
//             }
//         };
//         assert_eq!(actual, expected_result);
//     }
// }
