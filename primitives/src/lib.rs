#![cfg_attr(not(feature = "std"), no_std)]

use sp_runtime::{
    generic,
    traits::{BlakeTwo256, IdentifyAccount, Verify},
    MultiSignature
};

/// Alias to 512-bit hash when used in the context of a transaction signature on the chain.
pub type Signature = MultiSignature;

/// Some way of identifying an account on the chain. We intentionally make it equivalent
/// to the public key of our transaction signing scheme.
pub type AccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;

/// The type for looking up accounts. We don't expect more than 4 billion of them, but you
/// never know...
pub type AccountIndex = u32;

/// Balance of an account.
pub type Balance = u128;

/// Header type.
pub type Header = generic::Header<BlockNumber, BlakeTwo256>;

/// Block type.
pub type Block = generic::Block<Header, UncheckedExtrinsic>;

/// An index to a block.
pub type BlockNumber = u32;

/// Block ID.
pub type BlockId = generic::BlockId<Block>;

/// Digest item type.
pub type DigestItem = generic::DigestItem<Hash>;

/// A hash of some data used by the chain.
pub type Hash = sp_core::H256;

/// Index of a transaction in the chain.
pub type Index = u32;

/// An instant or duration in time.
pub type Moment = u64;

/// Opaque, encoded, unchecked extrinsic.
pub use sp_runtime::OpaqueExtrinsic as UncheckedExtrinsic;

pub mod currency {
    use super::*;
    pub const DOLLARS: Balance = 1_000_000_000_000_000_000;
    pub const CENTS: Balance = DOLLARS / 100; // 10_000_000_000_000_000
    pub const MILLICENTS: Balance = CENTS / 1000; // 10_000_000_000_000
    pub const MICROCENTS: Balance = MILLICENTS / 1000; // 10_000_000_000
}

pub mod time {
    use super::{BlockNumber, Moment};

    pub const SECS_PER_BLOCK: Moment = 4;
    pub const MILLISECS_PER_BLOCK: Moment = SECS_PER_BLOCK * 1000;

    // These time units are defined in number of blocks.
    pub const MINUTES: BlockNumber = 60 / (SECS_PER_BLOCK as BlockNumber);
    pub const HOURS: BlockNumber = MINUTES * 60;
    pub const DAYS: BlockNumber = HOURS * 24;

    pub const SLOT_DURATION: Moment = MILLISECS_PER_BLOCK;

    // 1 in 4 blocks (on average, not counting collisions) will be primary BABE blocks.
    pub const PRIMARY_PROBABILITY: (u64, u64) = (1, 4);

    pub const EPOCH_DURATION_IN_BLOCKS: BlockNumber = HOURS;
    pub const EPOCH_DURATION_IN_SLOTS: u64 = {
        const SLOT_FILL_RATE: f64 = MILLISECS_PER_BLOCK as f64 / SLOT_DURATION as f64;
        (EPOCH_DURATION_IN_BLOCKS as f64 * SLOT_FILL_RATE) as u64
    };
}
