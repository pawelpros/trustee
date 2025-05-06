use anyhow::{Error, Result};
use crate::eventlog::cclog::EventDetails;

pub mod parsers;


pub trait DescriptionParser: Sync + Send {
    fn parse_description(&self, data: Vec<u8>) -> Result<EventDetails, Error>;
}
