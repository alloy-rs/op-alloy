use alloy_consensus::{Block, BlockHeader, Transaction, EMPTY_ROOT_HASH};
use alloy_primitives::B256;
use alloy_rpc_types_engine::{CancunPayloadFields, MaybeCancunPayloadFields};
use derive_more::{Constructor, From, Into};

/// Container type for all available additional `newPayload` request parameters that are not present
/// in the [`ExecutionPayload`](alloy_rpc_types_engine::ExecutionPayload) object itself.
///
/// Default is equivalent to pre-cancun, payloads v1 and v2.
#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct OpExecutionPayloadSidecar {
    /// Cancun request params introduced in `engine_newPayloadV3` that are not present in the
    /// [`ExecutionPayload`](alloy_rpc_types_engine::ExecutionPayload).
    cancun: MaybeCancunPayloadFields,
    /// Storage root of `L2ToL1MessagePasser.sol`, aka l2 withdrawals root, requires state to
    /// compute, hence root is passed in sidecar.
    ///
    /// <https://specs.optimism.io/protocol/isthmus/exec-engine.html#update-to-executabledata>
    isthmus: MaybeIsthmusPayloadFields,
}

/// Fields introduced in `engine_newPayloadV4` that are not present in the
/// [`ExecutionPayload`](alloy_rpc_types_engine::ExecutionPayload) RPC object.
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash, Constructor)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct IsthmusPayloadFields {
    /// EIP-7685 requests.
    pub withdrawals_root: B256,
}

/// A container type for [`IsthmusPayloadFields`] that may or may not be present.
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash, From, Into)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[from(IsthmusPayloadFields)]
pub struct MaybeIsthmusPayloadFields {
    fields: Option<IsthmusPayloadFields>,
}

impl OpExecutionPayloadSidecar {
    /// Extracts the [`OpExecutionPayloadSidecar`] from the given [`Block`].
    ///
    /// Returns `OpExecutionPayloadSidecar::default` if the block does not contain any sidecar
    /// fields (pre-cancun): `requests_hash`, `parent_beacon_block_root`, `blob_versioned_hashes`.
    pub fn from_block<T, H>(block: &Block<T, H>) -> Self
    where
        T: Transaction,
        H: BlockHeader,
    {
        let cancun =
            block.parent_beacon_block_root().map(|parent_beacon_block_root| CancunPayloadFields {
                parent_beacon_block_root,
                versioned_hashes: block.body.blob_versioned_hashes_iter().copied().collect(),
            });

        let isthmus = block
            .withdrawals_root()
            .filter(|root| *root != EMPTY_ROOT_HASH)
            .map(IsthmusPayloadFields::new);

        match (cancun, isthmus) {
            (Some(cancun), Some(isthmus)) => Self::v4(cancun, isthmus),
            (Some(cancun), None) => Self::v3(cancun),
            _ => Self::default(),
        }
    }

    /// Creates a new instance for cancun with the cancun fields for `engine_newPayloadV3`
    pub fn v3(cancun: CancunPayloadFields) -> Self {
        Self { cancun: cancun.into(), ..Default::default() }
    }

    /// Creates a new instance post prague for `engine_newPayloadV4`
    pub fn v4(cancun: CancunPayloadFields, isthmus: IsthmusPayloadFields) -> Self {
        Self { cancun: cancun.into(), isthmus: isthmus.into() }
    }

    /// Returns a reference to the [`CancunPayloadFields`].
    pub const fn cancun(&self) -> Option<&CancunPayloadFields> {
        self.cancun.as_ref()
    }

    /// Consumes the type and returns the [`CancunPayloadFields`]
    pub fn into_cancun(self) -> Option<CancunPayloadFields> {
        self.cancun.into_inner()
    }

    /// Returns a reference to the [`IsthmusPayloadFields`].
    pub const fn isthmus(&self) -> Option<&IsthmusPayloadFields> {
        self.isthmus.fields.as_ref()
    }

    /// Consumes the type and returns the [`IsthmusPayloadFields`].
    pub fn into_isthmus(self) -> Option<IsthmusPayloadFields> {
        self.isthmus.into()
    }

    /// Returns the parent beacon block root, if any.
    pub fn parent_beacon_block_root(&self) -> Option<B256> {
        self.cancun.parent_beacon_block_root()
    }
}
