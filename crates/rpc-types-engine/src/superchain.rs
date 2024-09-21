use alloy_primitives::B64;

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
/// protocol version. The version must be encoded as 32 bytes of DATA in JSON RPC usage.
///
/// See also: <https://specs.optimism.io/protocol/superchain-upgrades.html#protocol-version>
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum ProtocolVersion {
    /// Version-type 0.
    V0(ProtocolVersionFormatV0),
}

#[cfg(feature = "std")]
#[derive(Copy, Clone, Debug, thiserror::Error)]
pub enum ProtocolVersionError {
    #[error("Unsupported version: {0}")]
    UnsupportedVersion(u8),
    #[error("Invalid version format length. Got {0}, expected 31")]
    InvalidLength(usize),
    #[error("Invalid version format encoding")]
    FromSlice(#[from] core::array::TryFromSliceError),
}

#[cfg(feature = "std")]
impl From<ProtocolVersion> for alloy_primitives::B256 {
    fn from(value: ProtocolVersion) -> alloy_primitives::B256 {
        let mut bytes = [0u8; 32];

        // <protocol version> ::= <version-type><typed-payload>
        // <version-type> ::= <uint8>
        // <typed-payload> ::= <31 bytes>
        match value {
            ProtocolVersion::V0(value) => {
                bytes[0] = 0x00; // this is not necessary, but addded for clarity
                bytes[1..].copy_from_slice(&value.into_slice());
                alloy_primitives::B256::from_slice(&bytes)
            }
        }
    }
}

#[cfg(feature = "std")]
impl TryFrom<alloy_primitives::B256> for ProtocolVersion {
    type Error = ProtocolVersionError;

    fn try_from(value: alloy_primitives::B256) -> Result<Self, Self::Error> {
        // <protocol version> ::= <version-type><typed-payload>
        // <version-type> ::= <uint8>
        // <typed-payload> ::= <31 bytes>
        let version_type = value[0];
        let typed_payload = &value[1..];

        match version_type {
            0 => Ok(Self::V0(ProtocolVersionFormatV0::try_from_slice(typed_payload)?)),
            other => Err(ProtocolVersionError::UnsupportedVersion(other)),
        }
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for ProtocolVersion {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        alloy_primitives::B256::from(*self).serialize(serializer)
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for ProtocolVersion {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = alloy_primitives::B256::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
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

#[cfg(feature = "std")]
impl std::fmt::Display for ProtocolVersionFormatV0 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{build}.{major}.{minor}.{patch}-{pre_release}",
            build = self.build,
            major = self.major,
            minor = self.minor,
            patch = self.patch,
            pre_release = self.pre_release
        )
    }
}

#[cfg(feature = "std")]
impl ProtocolVersionFormatV0 {
    /// Version-type 0 byte encoding:
    ///
    /// ```text
    /// <reserved><build><major><minor><patch><pre-release>
    /// <reserved> ::= <7 zeroed bytes>
    /// <build> ::= <8 bytes>
    /// <major> ::= <big-endian uint32>
    /// <minor> ::= <big-endian uint32>
    /// <patch> ::= <big-endian uint32>
    /// <pre-release> ::= <big-endian uint32>
    /// ```
    pub fn into_slice(self) -> [u8; 31] {
        let mut bytes = [0u8; 31];
        bytes[0..7].copy_from_slice(&[0u8; 7]);
        bytes[7..15].copy_from_slice(&self.build.0);
        bytes[15..19].copy_from_slice(&self.major.to_be_bytes());
        bytes[19..23].copy_from_slice(&self.minor.to_be_bytes());
        bytes[23..27].copy_from_slice(&self.patch.to_be_bytes());
        bytes[27..31].copy_from_slice(&self.pre_release.to_be_bytes());
        bytes
    }

    /// Version-type 0 byte encoding:
    ///
    /// ```text
    /// <reserved><build><major><minor><patch><pre-release>
    /// <reserved> ::= <7 zeroed bytes>
    /// <build> ::= <8 bytes>
    /// <major> ::= <big-endian uint32>
    /// <minor> ::= <big-endian uint32>
    /// <patch> ::= <big-endian uint32>
    /// <pre-release> ::= <big-endian uint32>
    /// ```
    fn try_from_slice(value: &[u8]) -> Result<Self, ProtocolVersionError> {
        if value.len() != 31 {
            return Err(ProtocolVersionError::InvalidLength(value.len()));
        }

        Ok(Self {
            build: B64::from_slice(&value[7..15]),
            major: u32::from_be_bytes(value[15..19].try_into()?),
            minor: u32::from_be_bytes(value[19..23].try_into()?),
            patch: u32::from_be_bytes(value[23..27].try_into()?),
            pre_release: u32::from_be_bytes(value[27..31].try_into()?),
        })
    }
}
