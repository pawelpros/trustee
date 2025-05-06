use num_enum::TryFromPrimitive;
use serde::Serialize;
use crate::eventlog::cclog::parser::DescriptionParser;
use crate::eventlog::cclog::parser::parsers::*;

#[repr(u32)]
#[derive(Debug, Clone, Hash, Copy, PartialEq, Eq, TryFromPrimitive, Serialize)]
pub enum TcgAlgorithm {
    #[serde(rename = "RSA")]
    Rsa = 0x1,
    #[serde(rename = "TDES")]
    Tdes = 0x3,
    #[serde(rename = "SHA-1")]
    Sha1 = 0x4,
    #[serde(rename = "SHA-256")]
    Sha256 = 0xB,
    #[serde(rename = "SHA-384")]
    Sha384 = 0xC,
    #[serde(rename = "SHA-512")]
    Sha512 = 0xD,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, TryFromPrimitive)]
pub enum TcgEventType {
    EvPrebootCert = 0x0,
    EvPostCode = 0x1,
    EvUnused = 0x2,
    EvNoAction = 0x3,
    EvSeparator = 0x4,
    EvAction = 0x5,
    EvEventTag = 0x6,
    EvSCrtmContents = 0x7,
    EvSCrtmVersion = 0x8,
    EvCpuMicrocode = 0x9,
    EvPlatformConfigFlags = 0xa,
    EvTableOfDevices = 0xb,
    EvCompactHash = 0xc,
    EvIpl = 0xd,
    EvIplPartitionData = 0xe,
    EvNonhostCode = 0xf,
    EvNonhostConfig = 0x10,
    EvNonhostInfo = 0x11,
    EvOmitBootDeviceEvents = 0x12,

    // TCG EFI Platform Specification For TPM Family 1.1 or 1.2
    EvEfiEventBase = 0x80000000,
    EvEfiVariableDriverConfig = 0x80000001,
    EvEfiVariableBoot = 0x80000002,
    EvEfiBootServicesApplication = 0x80000003,
    EvEfiBootServicesDriver = 0x80000004,
    EvEfiRuntimeServicesDriver = 0x80000005,
    EvEfiGptEvent = 0x80000006,
    EvEfiAction = 0x80000007,
    EvEfiPlatformFirmwareBlob = 0x80000008,
    EvEfiHandoffTables = 0x80000009,
    EvEfiPlatformFirmwareBlob2 = 0x8000000a,
    EvEfiHandoffTables2 = 0x8000000b,
    EvEfiVariableBoot2 = 0x8000000c,
    EvEfiHcrtmEvent = 0x80000010,
    EvEfiVariableAuthority = 0x800000e0,
    EvEfiSpdmFirmwareBlob = 0x800000e1,
    EvEfiSpdmFirmwareConfig = 0x800000e2,
}

impl TcgEventType {
    pub(crate) fn get_parser(&self) -> Box<dyn DescriptionParser> {
        match self {
            Self::EvPostCode => Box::new(EvSimpleParser),
            Self::EvSeparator => Box::new(EvBlankParser),
            Self::EvAction => Box::new(EvSimpleParser),
            Self::EvEventTag => Box::new(EvEventTagParser),
            Self::EvPlatformConfigFlags => Box::new(EvSimpleParser),
            Self::EvCompactHash => Box::new(EvSimpleParser),
            Self::EvIpl => Box::new(EvSimpleParser),
            Self::EvOmitBootDeviceEvents => Box::new(EvSimpleParser),
            Self::EvEfiVariableDriverConfig => Box::new(EvEfiVariableParser),
            Self::EvEfiVariableBoot => Box::new(EvEfiVariableParser),
            Self::EvEfiBootServicesApplication => Box::new(EvBootServicesAppParser),
            Self::EvEfiAction => Box::new(EvSimpleParser),
            Self::EvEfiPlatformFirmwareBlob2 => Box::new(SimpleStringParser),
            Self::EvEfiHandoffTables2 => Box::new(SimpleStringParser),
            Self::EvEfiVariableBoot2 => Box::new(EvEfiVariableParser),
            Self::EvEfiVariableAuthority => Box::new(EvEfiVariableParser),
            _ => Box::new(EvBlankParser),
        }
    }

    pub fn format_name(&self) -> String {
        let name = format!("{:?}", self);

        let mut result = String::new();
        for (i, ch) in name.chars().enumerate() {
            if ch.is_uppercase() && i > 0 {
                result.push('_');
            }
            result.push(ch.to_ascii_uppercase());
        }

        result
    }
}
