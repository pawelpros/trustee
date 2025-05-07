use crate::eventlog::cclog::{CcEventLog, EventlogEntry};
use anyhow::bail;
use anyhow::*;
use core::fmt;
use sha2::{Digest, Sha384};
use std::collections::HashMap;
use std::result::Result::Ok;

const RTMR_LENGTH_BY_BYTES: usize = 48;

#[derive(Debug, Clone, Copy)]
pub struct Rtmr {
    pub rtmr0: [u8; RTMR_LENGTH_BY_BYTES],
    pub rtmr1: [u8; RTMR_LENGTH_BY_BYTES],
    pub rtmr2: [u8; RTMR_LENGTH_BY_BYTES],
    pub rtmr3: [u8; RTMR_LENGTH_BY_BYTES],
}

impl Rtmr {
    pub fn integrity_check(&self, rtmr_from_quote: Rtmr) -> Result<()> {
        // Compare rtmr values from tdquote and EventLog acpi table
        if rtmr_from_quote.rtmr0 != self.rtmr0
            || rtmr_from_quote.rtmr1 != self.rtmr1
            || rtmr_from_quote.rtmr2 != self.rtmr2
        {
            bail!("RTMR 0, 1, 2 values from TD quote is not equal with the values from EventLog");
        }

        Ok(())
    }
}

impl fmt::Display for Rtmr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "RTMR[0]: {}", hex::encode(self.rtmr0))?;
        writeln!(f, "RTMR[1]: {}", hex::encode(self.rtmr1))?;
        writeln!(f, "RTMR[2]: {}", hex::encode(self.rtmr2))?;
        writeln!(f, "RTMR[3]: {}", hex::encode(self.rtmr3))?;
        Ok(())
    }
}

impl TryFrom<CcEventLog> for Rtmr {
    type Error = anyhow::Error;

    fn try_from(data: CcEventLog) -> anyhow::Result<Self> {
        let mr_map = replay_measurement_registry(data);

        let mr = Rtmr {
            rtmr0: mr_map
                .get(&1)
                .unwrap_or(&Vec::from([0u8; RTMR_LENGTH_BY_BYTES]))[0..RTMR_LENGTH_BY_BYTES]
                .try_into()?,
            rtmr1: mr_map
                .get(&2)
                .unwrap_or(&Vec::from([0u8; RTMR_LENGTH_BY_BYTES]))[0..RTMR_LENGTH_BY_BYTES]
                .try_into()?,
            rtmr2: mr_map
                .get(&3)
                .unwrap_or(&Vec::from([0u8; RTMR_LENGTH_BY_BYTES]))[0..RTMR_LENGTH_BY_BYTES]
                .try_into()?,
            rtmr3: mr_map
                .get(&4)
                .unwrap_or(&Vec::from([0u8; RTMR_LENGTH_BY_BYTES]))[0..RTMR_LENGTH_BY_BYTES]
                .try_into()?,
        };

        Ok(mr)
    }
}

fn replay_measurement_registry(data: CcEventLog) -> HashMap<u32, Vec<u8>> {
    let mut event_logs_by_mr_index: HashMap<u32, Vec<EventlogEntry>> = HashMap::new();

    let mut result: HashMap<u32, Vec<u8>> = HashMap::new();

    for log_entry in data.log.iter() {
        match event_logs_by_mr_index.get_mut(&log_entry.index) {
            Some(logs) => logs.push(log_entry.clone()),
            None => {
                event_logs_by_mr_index.insert(log_entry.index, vec![log_entry.clone()]);
            }
        }
    }

    for (mr_index, log_set) in event_logs_by_mr_index.iter() {
        let mut mr_value = [0; RTMR_LENGTH_BY_BYTES];

        for log in log_set.iter() {
            let digest = &log.digests[0].digest;
            let mut sha384_algo = Sha384::new();
            sha384_algo.update(mr_value);
            sha384_algo.update(digest.as_slice());
            mr_value.copy_from_slice(sha384_algo.finalize().as_slice());
        }
        result.insert(*mr_index, mr_value.to_vec());
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    use std::fs;

    #[rstest]
    #[case("./test_data/CCEL_data")]
    #[case("./test_data/CCEL_data_ovmf")]
    #[case("./test_data/CCEL_data_grub")]
    fn test_rebuild_rtmr(#[case] test_data: &str) {
        let ccel_bin = fs::read(test_data).unwrap();
        let ccel = CcEventLog::try_from(ccel_bin).unwrap();
        let rtmr_result = Rtmr::try_from(ccel);
        assert!(rtmr_result.is_ok());
    }
}
