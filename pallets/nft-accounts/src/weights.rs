//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 3.0.0

#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(clippy::unnecessary_cast)]

use frame_support::weights::{constants::RocksDbWeight as DbWeight, Weight};
use sp_std::marker::PhantomData;

impl crate::WeightInfo for () {
	fn claim_account() -> Weight {
		(1_253_760_000 as Weight)
			.saturating_add(DbWeight::get().reads(3 as Weight))
			.saturating_add(DbWeight::get().writes(4 as Weight))
	}
}

pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> crate::WeightInfo for WeightInfo<T> {
	fn claim_account() -> Weight {
		(1_253_760_000 as Weight)
			.saturating_add(DbWeight::get().reads(3 as Weight))
			.saturating_add(DbWeight::get().writes(4 as Weight))
	}
}
