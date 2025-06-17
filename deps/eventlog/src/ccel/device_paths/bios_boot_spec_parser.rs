use super::DevicePathParser;
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
