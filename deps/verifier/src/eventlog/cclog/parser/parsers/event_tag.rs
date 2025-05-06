use anyhow::{Error, Result};
use crate::eventlog::cclog::{utils, EventDetails};
use crate::eventlog::cclog::parser::DescriptionParser;

pub struct EvEventTagParser;
impl DescriptionParser for EvEventTagParser {
    fn parse_description(&self, data: Vec<u8>) -> Result<EventDetails, Error> {
        let mut index = 4;
        let length = utils::read_u32_le(&data, &mut index)? as usize;

        let description_bytes =
            utils::get_next_bytes(&*data, &mut index, length)?;

        let event_desc = String::from_utf8(description_bytes.to_vec())?
            .replace('\0', "");
        Ok(EventDetails::from_string(event_desc))
    }
}
