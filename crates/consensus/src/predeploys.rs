//! Addresses of OP Stack pre-deployed contracts.
//!
//! These are the addresses of contracts that are pre-deployed on every OP Stack chain.
//! See <https://specs.optimism.io/protocol/predeploys.html> for more details.

use alloy_primitives::{address, Address};

/// The `L2ToL1MessagePasser` predeploy address.
///
/// This contract stores commitments to withdrawal transactions and is used for
/// L2 to L1 message passing.
///
/// Address: `0x4200000000000000000000000000000000000016`
pub const L2_TO_L1_MESSAGE_PASSER: Address =
    address!("0x4200000000000000000000000000000000000016");
