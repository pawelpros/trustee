use anyhow::Error;
use crate::eventlog::cclog::EventDetails;
use crate::eventlog::cclog::parser::DescriptionParser;

pub struct EvBlankParser;
impl DescriptionParser for EvBlankParser {
    fn parse_description(&self, _data: Vec<u8>) -> anyhow::Result<EventDetails, Error> {
        Ok(EventDetails::empty())
    }
}
