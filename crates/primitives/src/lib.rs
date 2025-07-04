//! Commonly used types in Reth.
//!
//! This crate contains Ethereum primitive types and helper functions.
//!
//! ## Feature Flags
//!
//! - `arbitrary`: Adds `proptest` and `arbitrary` support for primitive types.
//! - `test-utils`: Export utilities for testing
//! - `reth-codec`: Enables db codec support for reth types including zstd compression for certain
//!   types.

#![doc(
    html_logo_url = "https://raw.githubusercontent.com/paradigmxyz/reth/main/assets/reth-docs.png",
    html_favicon_url = "https://avatars0.githubusercontent.com/u/97369466?s=256",
    issue_tracker_base_url = "https://github.com/paradigmxyz/reth/issues/"
)]
#![cfg_attr(not(test), warn(unused_crate_dependencies))]
#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]
#![cfg_attr(not(feature = "std"), no_std)]

mod block;
mod receipt;
pub use reth_static_file_types as static_file;
pub mod transaction;
#[cfg(any(test, feature = "arbitrary"))]
pub use block::{generate_valid_header, valid_header_strategy};
pub use block::{Block, BlockBody, SealedBlock};
#[expect(deprecated)]
pub use block::{BlockWithSenders, SealedBlockFor, SealedBlockWithSenders};

pub use receipt::{gas_spent_by_transactions, Receipt};
pub use reth_primitives_traits::{
    logs_bloom, Account, BlockTy, BodyTy, Bytecode, GotExpected, GotExpectedBoxed, Header,
    HeaderTy, Log, LogData, NodePrimitives, ReceiptTy, RecoveredBlock, SealedHeader, StorageEntry,
    TxTy,
};
pub use static_file::StaticFileSegment;

pub use alloy_consensus::{
    transaction::{PooledTransaction, Recovered, TransactionMeta},
    ReceiptWithBloom,
};

/// Recovered transaction
#[deprecated(note = "use `Recovered` instead")]
pub type RecoveredTx<T> = Recovered<T>;

pub use transaction::{
    util::secp256k1::{public_key_to_address, recover_signer_unchecked, sign_message},
    InvalidTransactionError, Transaction, TransactionSigned, TxType,
};
#[expect(deprecated)]
pub use transaction::{PooledTransactionsElementEcRecovered, TransactionSignedEcRecovered};

// Re-exports
pub use reth_ethereum_forks::*;

#[cfg(any(test, feature = "arbitrary"))]
pub use arbitrary;

#[cfg(feature = "c-kzg")]
pub use c_kzg as kzg;

/// Bincode-compatible serde implementations for commonly used types in Reth.
///
/// `bincode` crate doesn't work with optionally serializable serde fields, but some of the
/// Reth types require optional serialization for RPC compatibility. This module makes so that
/// all fields are serialized.
///
/// Read more: <https://github.com/bincode-org/bincode/issues/326>
#[cfg(feature = "serde-bincode-compat")]
pub mod serde_bincode_compat {
    pub use reth_primitives_traits::serde_bincode_compat::*;
}

// Re-export of `EthPrimitives`
pub use reth_ethereum_primitives::EthPrimitives;
