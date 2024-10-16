//! Contains the [BatchValidity] and its encodings.

/// Batch Validity
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BatchValidity {
    /// The batch is invalid now and in the future, unless we reorg
    Drop,
    /// The batch is valid and should be processed
    Accept,
    /// We are lacking L1 information until we can proceed batch filtering
    Undecided,
    /// The batch may be valid, but cannot be processed yet and should be checked again later
    Future,
    /// Introduced in Holocene, a special variant of the Drop variant that signals not to flush
    /// the active batch and channel, in the case of processing an old batch
    Past,
}

impl BatchValidity {
    /// Returns if the batch is accepted.
    pub const fn is_accept(&self) -> bool {
        matches!(self, Self::Accept)
    }

    /// Returns if the batch is dropped.
    pub const fn is_drop(&self) -> bool {
        matches!(self, Self::Drop)
    }

    /// Returns if the batch is outdated.
    pub const fn is_outdated(&self) -> bool {
        matches!(self, Self::Past)
    }

    /// Returns if the batch is future.
    pub const fn is_future(&self) -> bool {
        matches!(self, Self::Future)
    }
}