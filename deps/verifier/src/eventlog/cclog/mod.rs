use anyhow::{anyhow, Result};
use base64::{engine::general_purpose::STANDARD, Engine};
use serde::ser::{SerializeSeq, SerializeStruct};
use serde::{Serialize, Serializer};
use serde_json::Value;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::u32;
use crate::eventlog::cclog::tcg_enum::{TcgEventType, TcgAlgorithm};

pub mod rtmr;
pub mod tcg_enum;

mod parser;
mod utils;

#[derive(Clone, Serialize)]
pub struct Eventlog {
    #[serde(rename = "uefi_event_logs")]
    pub log: Vec<EventlogEntry>,
}

#[derive(Debug, Clone)]
pub struct EventlogEntry {
    pub index: u32,
    pub event_type: TcgEventType,
    pub digests: Vec<ElDigest>,
    pub event: String,
    pub details: EventDetails,
}

#[derive(Debug, Clone, Serialize)]
pub struct EventDetails {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub string: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unicode_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unicode_name_length: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variable_data: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variable_data_length: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variable_name: Option<String>,
    #[serde(
        serialize_with = "serialize_json_string_vec",
        skip_serializing_if = "Option::is_none"
    )]
    pub data: Option<Vec<String>>, // TODO NOT FULLY IMPLEMENTED AS ITA
}

impl EventDetails {
    pub fn from_string(s: String) -> Self {
        Self {
            string: Some(s),
            unicode_name: None,
            unicode_name_length: None,
            variable_data: None,
            variable_data_length: None,
            variable_name: None,
            data: None,
        }
    }

    pub fn empty() -> Self {
        Self {
            string: None,
            unicode_name: None,
            unicode_name_length: None,
            variable_data: None,
            variable_data_length: None,
            variable_name: None,
            data: None,
        }
    }
}

impl Serialize for EventlogEntry {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("EventlogEntry", 6)?;
        state.serialize_field("details", &self.details)?;
        state.serialize_field("digests", &self.digests)?;
        state.serialize_field("event", &self.event)?;
        state.serialize_field("index", &self.index)?;
        // state.serialize_field("type", &format!("0x{:08X}", self.event_type as u32))?; // TODO ITA DIFFERENCE
        state.serialize_field("type", &(self.event_type as u32))?;
        state.serialize_field("type_name", &self.event_type.format_name())?;
        state.end()
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct ElDigest {
    pub alg: TcgAlgorithm,
    #[serde(serialize_with = "serialize_digest_as_hex")]
    pub digest: Vec<u8>,
}

fn serialize_digest_as_hex<S>(digest: &Vec<u8>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&hex::encode(digest))
}

pub fn serialize_json_string_vec<S>(
    vec: &Option<Vec<String>>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match vec {
        Some(inner_vec) => {
            let mut seq = serializer.serialize_seq(Some(inner_vec.len()))?;
            for json_str in inner_vec {
                let json_value: Value =
                    serde_json::from_str(json_str).map_err(serde::ser::Error::custom)?;
                seq.serialize_element(&json_value)?;
            }
            seq.end()
        }
        None => serializer.serialize_none(),
    }
}

impl TryFrom<Vec<u8>> for Eventlog {
    type Error = anyhow::Error;

    fn try_from(data: Vec<u8>) -> Result<Self> {
        let mut index = 0;
        let mut event_log = Vec::new();
        let mut digest_size_map = HashMap::new();

        while index < data.len() {
            let entry_opt;
            (entry_opt, index) = parse_eventlog_entry(&data, index, &mut digest_size_map)?;
            if let Some(entry) = entry_opt {
                event_log.push(entry);
            } else if index == 0 {
                break;
            }
        }

        Ok(Eventlog { log: event_log })
    }
}

fn parse_eventlog_entry(
    data: &[u8],
    mut index: usize,
    digest_size_map: &mut HashMap<TcgAlgorithm, u16>,
) -> Result<(Option<EventlogEntry>, usize)> {
    let stop_flag = utils::read_u64_le(data, &mut index)?;
    index -= size_of::<u64>();
    if stop_flag == 0xFFFFFFFFFFFFFFFF || stop_flag == 0x0000000000000000 {
        return Ok((None, 0));
    }

    let target_measurement_registry = utils::read_u32_le(data, &mut index)?;

    let event_type_num = utils::read_u32_le(data, &mut index)?;

    let event_type = TcgEventType::try_from(event_type_num)
        .map_err(|_| anyhow!("Unknown event type detected: {:#x}", event_type_num))?;

    if event_type == TcgEventType::EvNoAction {
        index = parse_digest_sizes(data, index, digest_size_map)?;
        return Ok((None, index));
    }

    let digests;
    (digests, index) = parse_digests(data, index, digest_size_map)?;

    let event_desc_size = utils::read_u32_le(data, &mut index)?;
    let event_desc_raw = data[index..(index + event_desc_size as usize)].to_vec();
    index += event_desc_size as usize;

    let event = STANDARD.encode(&event_desc_raw); // TODO USE THIS ONE
                                                  // let event = hex::encode(&event_desc_raw);
    let event_result = event_type.get_parser().parse_description(event_desc_raw)?;

    Ok((
        Some(EventlogEntry {
            index: target_measurement_registry,
            event_type,
            digests,
            event,
            details: event_result,
        }),
        index,
    ))
}

fn parse_digest_sizes(
    data: &[u8],
    mut index: usize,
    digest_size_map: &mut HashMap<TcgAlgorithm, u16>,
) -> Result<usize> {
    index += 48;
    let algo_number = utils::read_u32_le(data, &mut index)?;

    for _ in 0..algo_number {
        let algo_id = utils::read_u16_le(data, &mut index)?;
        let size = utils::read_u16_le(data, &mut index)?;

        let algorithm = TcgAlgorithm::try_from(algo_id as u32)
            .map_err(|_| anyhow!("Unknown algorithm type detected: {:x}", algo_id))?;

        digest_size_map.insert(algorithm, size);
    }

    let vendor_size = data[index] as usize;
    index += vendor_size + 1;
    Ok(index)
}

fn parse_digests(
    data: &[u8],
    mut index: usize,
    digest_size_map: &HashMap<TcgAlgorithm, u16>,
) -> Result<(Vec<ElDigest>, usize)> {
    let digest_count = utils::read_u32_le(data, &mut index)?;

    let mut digests = Vec::new();
    for _ in 0..digest_count {
        let algo_id;
        algo_id = utils::read_u16_le(data, &mut index)?;

        let algorithm = TcgAlgorithm::try_from(algo_id as u32)
            .map_err(|_| anyhow!("Unknown algorithm type detected: {:x}", algo_id))?;

        let size = *digest_size_map
            .get(&algorithm)
            .ok_or_else(|| anyhow!("Missing digest size for algorithm: {:x}", algo_id))?
            as usize;

        let digest = data[index..index + size].to_vec();
        index += size;

        digests.push(ElDigest {
            alg: algorithm,
            digest,
        });
    }

    Ok((digests, index))
}
