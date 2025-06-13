use super::{DevicePathParser};
use anyhow::{bail, Result};

pub struct BiosBootSpecParser;

impl DevicePathParser for BiosBootSpecParser {
    fn parse(&self, sub_type: u8, _data: &[u8]) -> Result<String> {
        // match sub_type {
        //     0x01 => bss(data),
        //     _ => { bail!("Unknown sub type {}", sub_type) }
        // }
        bail!("sub type {} not implemented", sub_type)
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
