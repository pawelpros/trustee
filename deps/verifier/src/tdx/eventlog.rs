use crate::eventlog::cclog::rtmr::Rtmr;
use crate::eventlog::cclog::Eventlog;
use anyhow::*;
use std::result::Result::Ok;

#[derive(Clone)]
pub struct CcEventLog {
    pub cc_events: Eventlog,
}

impl TryFrom<Vec<u8>> for CcEventLog {
    type Error = anyhow::Error;
    fn try_from(data: Vec<u8>) -> Result<Self, Self::Error> {
        Ok(Self {
            cc_events: Eventlog::try_from(data)?,
        })
    }
}

impl CcEventLog {
    pub fn integrity_check(&self, rtmr_from_quote: Rtmr) -> Result<()> {
        let rtmr_eventlog = Rtmr::try_from(self.cc_events.clone())?;

        // Compare rtmr values from tdquote and EventLog acpi table
        if rtmr_from_quote.rtmr0 != rtmr_eventlog.rtmr0
            || rtmr_from_quote.rtmr1 != rtmr_eventlog.rtmr1
            || rtmr_from_quote.rtmr2 != rtmr_eventlog.rtmr2
        {
            bail!("RTMR 0, 1, 2 values from TD quote is not equal with the values from EventLog");
        }

        Ok(())
    }
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
        let rtmr_result = Rtmr::try_from(ccel.cc_events);
        assert!(rtmr_result.is_ok());
    }
}
