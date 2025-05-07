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

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use rstest::rstest;
//     use std::fs;
//
//     #[rstest]
//     #[case("./test_data/CCEL_data")]
//     #[case("./test_data/CCEL_data_ovmf")]
//     #[case("./test_data/CCEL_data_grub")]
//     fn test_rebuild_rtmr(#[case] test_data: &str) {
//         let ccel_bin = fs::read(test_data).unwrap();
//         let ccel = CcEventLog::try_from(ccel_bin).unwrap();
//         let rtmr_result = Rtmr::try_from(ccel.cc_events);
//         assert!(rtmr_result.is_ok());
//     }
//
//     #[rstest]
//     #[case("./test_data/CCEL_data", MeasuredEntity::TdShimKernel, String::from("5b7aa6572f649714ff00b6a2b9170516a068fd1a0ba72aa8de27574131d454e6396d3bfa1727d9baf421618a942977fa"))]
//     #[case("./test_data/CCEL_data", MeasuredEntity::TdShimKernelParams, String::from("64ed1e5a47e8632f80faf428465bd987af3e8e4ceb10a5a9f387b6302e30f4993bded2331f0691c4a38ad34e4cbbc627"))]
//     #[case("./test_data/CCEL_data_ovmf", MeasuredEntity::TdvfKernel, String::from("a2ccae1e7d6c668ca325bb09c882d8ce44d26d714ba6f58d2e8083fe291a704646afe24a2368bca3341728d78ec80a80"))]
//     #[case("./test_data/CCEL_data_ovmf", MeasuredEntity::TdvfKernelParams, String::from("4230f84885a6f3f305e91a1955045398bd9edd8ffd2aaf2aab8ad3ac53476c4ac82a3675ef559c4ae949a06e84119fc2"))]
//     #[case("./test_data/CCEL_data_ovmf", MeasuredEntity::TdvfInitrd, String::from("b15af9286108d3d8c9f794a51409e55bad6334f5d96a1e4469f8df2d75fd69aac648d939e13daf6800e82e6c1f6628c4"))]
//     #[case("./test_data/CCEL_data_grub", MeasuredEntity::TdvfInitrd, String::from("15485f8c0ea5fb6c497e13830915858173d9c9558708cbbc7b26e52f6bbe7313b3fa772f6120d0815d0f4aa7dfc75888"))]
//     #[case("./test_data/CCEL_data_grub", MeasuredEntity::TdvfKernelParams, String::from("f45887f32c15f51f7a384ed851c22823097c29b79a44f80a598f7132ca80e02c419a1e8c6902fbd961d3a0225fccc034"))]
//     fn test_query_digest(
//         #[case] test_data: &str,
//         #[case] measured_entity: MeasuredEntity,
//         #[case] reference_digest: String,
//     ) {
//         let ccel_bin = fs::read(test_data).expect("open test data");
//         let ccel = CcEventLog::try_from(ccel_bin).expect("parse CCEL eventlog");
//
//         assert_eq!(
//             ccel.query_digest(measured_entity).unwrap(),
//             reference_digest
//         );
//     }
// }
