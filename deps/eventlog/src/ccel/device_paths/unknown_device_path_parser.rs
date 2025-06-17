use super::DevicePathParser;
use anyhow::{bail, Result};

pub struct UnknownDevicePathParser;

impl DevicePathParser for UnknownDevicePathParser {
    fn parse(&self, _sub_type: u8, _data: &[u8]) -> Result<String> {
        bail!("Unknown device path parser")
    }
}
