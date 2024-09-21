use core::array::TryFromSliceError;

use alloy_primitives::{B256, B64};
use cfg_if::cfg_if;

/// Superchain Signal information.
///
/// The execution engine SHOULD warn the user when the recommended version is newer than the current
/// version supported by the execution engine.
///
/// The execution engine SHOULD take safety precautions if it does not meet the required protocol
/// version. This may include halting the engine, with consent of the execution engine operator.
///
/// See also: <https://specs.optimism.io/protocol/exec-engine.html#engine_signalsuperchainv1>
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct SuperchainSignal {
    /// The recommended Supercain Protocol Version.
    pub recommended: ProtocolVersion,
    /// The minimum Supercain Protocol Version required.
    pub required: ProtocolVersion,
}

/// Formatted Superchain Protocol Version.
///
/// The Protocol Version documents the progression of the total set of canonical OP-Stack
/// specifications. Components of the OP-Stack implement the subset of their respective protocol
/// component domain, up to a given Protocol Version of the OP-Stack.
///
/// The Protocol Version **is NOT a hardfork identifier**, but rather indicates software-support for
/// a well-defined set of features introduced in past and future hardforks, not the activation of
/// said hardforks.
///
/// The Protocol Version is Semver-compatible. It is encoded as a single 32 bytes long
/// <protocol version>. The version must be encoded as 32 bytes of DATA in JSON RPC usage.
///
/// See also: <https://specs.optimism.io/protocol/superchain-upgrades.html#protocol-version>
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ProtocolVersion {
    /// Version-type 0.
    V0(ProtocolVersionFormatV0),
}

cfg_if! {
    if #[cfg(feature = "serde")] {
        use serde::{de, Serialize, Serializer, Deserialize, Deserializer};

        impl Serialize for ProtocolVersion {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                match self {
                    Self::V0(value) => {
                        // <protocol version> ::= <version-type><typed-payload>
                        // <version-type> ::= <uint8>
                        // <typed-payload> ::= <31 bytes>
                        let mut bytes = [0u8; 32];
                        bytes[0] = 0x00; // this is not necessary, but addded for clarity
                        bytes[1..].copy_from_slice(&Into::<[u8; 31]>::into(*value));

                        B256::from_slice(&bytes).serialize(serializer)
                    }
                }
            }
        }

        impl<'de> Deserialize<'de> for ProtocolVersion {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                // The version must be encoded as 32 bytes of DATA in JSON RPC usage.
                let value = B256::deserialize(deserializer)?;

                // <protocol version> ::= <version-type><typed-payload>
                // <version-type> ::= <uint8>
                // <typed-payload> ::= <31 bytes>
                let version_type = value[0];
                let typed_payload = &value[1..];

                match version_type {
                    0 => Ok(Self::V0(ProtocolVersionFormatV0::try_from(typed_payload).map_err(de::Error::custom)?)),
                    other => Err(de::Error::custom(format!("unsupported protocol version: {}", other))),
                }
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct ProtocolVersionFormatV0 {
    pub build: B64,
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
    pub pre_release: u32,
}

impl From<ProtocolVersionFormatV0> for [u8; 31] {
    fn from(value: ProtocolVersionFormatV0) -> Self {
        // Version-type 0:
        //
        // <reserved><build><major><minor><patch><pre-release>
        // <reserved> ::= <7 zeroed bytes>
        // <build> ::= <8 bytes>
        // <major> ::= <big-endian uint32>
        // <minor> ::= <big-endian uint32>
        // <patch> ::= <big-endian uint32>
        // <pre-release> ::= <big-endian uint32>

        let mut bytes = [0u8; 31];
        bytes[0..7].copy_from_slice(&[0u8; 7]);
        bytes[7..15].copy_from_slice(&value.build.0);
        bytes[15..19].copy_from_slice(&value.major.to_be_bytes());
        bytes[19..23].copy_from_slice(&value.minor.to_be_bytes());
        bytes[23..27].copy_from_slice(&value.patch.to_be_bytes());
        bytes[27..31].copy_from_slice(&value.pre_release.to_be_bytes());
        bytes
    }
}

impl TryFrom<&[u8]> for ProtocolVersionFormatV0 {
    type Error = TryFromSliceError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        // Version-type 0:
        //
        // <reserved><build><major><minor><patch><pre-release>
        // <reserved> ::= <7 zeroed bytes>
        // <build> ::= <8 bytes>
        // <major> ::= <big-endian uint32>
        // <minor> ::= <big-endian uint32>
        // <patch> ::= <big-endian uint32>
        // <pre-release> ::= <big-endian uint32>

        Ok(Self {
            build: B64::from_slice(&value[7..15]),
            major: u32::from_be_bytes(value[15..19].try_into()?),
            minor: u32::from_be_bytes(value[19..23].try_into()?),
            patch: u32::from_be_bytes(value[23..27].try_into()?),
            pre_release: u32::from_be_bytes(value[27..31].try_into()?),
        })
    }
}
