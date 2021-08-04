//! Autogenerated weights for pallet_identity
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2021-07-22, STEPS: `[50, ]`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 128

// Executed Command:
// target/release/nftmart-node
// benchmark
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=pallet_identity
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./node/runtime/src/weights

#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for pallet_identity.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_identity::WeightInfo for WeightInfo<T> {
	fn add_registrar(r: u32) -> Weight {
		(25_899_000 as Weight)
			// Standard Error: 5_000
			.saturating_add((525_000 as Weight).saturating_mul(r as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn set_identity(r: u32, x: u32) -> Weight {
		(61_604_000 as Weight)
			// Standard Error: 18_000
			.saturating_add((290_000 as Weight).saturating_mul(r as Weight))
			// Standard Error: 2_000
			.saturating_add((1_094_000 as Weight).saturating_mul(x as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn set_subs_new(s: u32) -> Weight {
		(51_129_000 as Weight)
			// Standard Error: 2_000
			.saturating_add((7_342_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(s as Weight)))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(s as Weight)))
	}
	fn set_subs_old(p: u32) -> Weight {
		(47_926_000 as Weight)
			// Standard Error: 1_000
			.saturating_add((2_509_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(p as Weight)))
	}
	fn clear_identity(r: u32, s: u32, x: u32) -> Weight {
		(60_114_000 as Weight)
			// Standard Error: 8_000
			.saturating_add((200_000 as Weight).saturating_mul(r as Weight))
			// Standard Error: 0
			.saturating_add((2_478_000 as Weight).saturating_mul(s as Weight))
			// Standard Error: 0
			.saturating_add((733_000 as Weight).saturating_mul(x as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(s as Weight)))
	}
	fn request_judgement(r: u32, x: u32) -> Weight {
		(63_050_000 as Weight)
			// Standard Error: 9_000
			.saturating_add((480_000 as Weight).saturating_mul(r as Weight))
			// Standard Error: 1_000
			.saturating_add((1_396_000 as Weight).saturating_mul(x as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn cancel_request(r: u32, x: u32) -> Weight {
		(59_017_000 as Weight)
			// Standard Error: 8_000
			.saturating_add((294_000 as Weight).saturating_mul(r as Weight))
			// Standard Error: 1_000
			.saturating_add((1_374_000 as Weight).saturating_mul(x as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn set_fee(r: u32) -> Weight {
		(10_751_000 as Weight)
			// Standard Error: 4_000
			.saturating_add((448_000 as Weight).saturating_mul(r as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn set_account_id(r: u32) -> Weight {
		(11_845_000 as Weight)
			// Standard Error: 4_000
			.saturating_add((457_000 as Weight).saturating_mul(r as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn set_fields(r: u32) -> Weight {
		(10_703_000 as Weight)
			// Standard Error: 4_000
			.saturating_add((451_000 as Weight).saturating_mul(r as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn provide_judgement(r: u32, x: u32) -> Weight {
		(41_535_000 as Weight)
			// Standard Error: 6_000
			.saturating_add((450_000 as Weight).saturating_mul(r as Weight))
			// Standard Error: 0
			.saturating_add((1_377_000 as Weight).saturating_mul(x as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn kill_identity(r: u32, s: u32, x: u32) -> Weight {
		(75_802_000 as Weight)
			// Standard Error: 11_000
			.saturating_add((137_000 as Weight).saturating_mul(r as Weight))
			// Standard Error: 1_000
			.saturating_add((2_476_000 as Weight).saturating_mul(s as Weight))
			// Standard Error: 1_000
			.saturating_add((2_000 as Weight).saturating_mul(x as Weight))
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(s as Weight)))
	}
	fn add_sub(s: u32) -> Weight {
		(65_111_000 as Weight)
			// Standard Error: 1_000
			.saturating_add((308_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	fn rename_sub(s: u32) -> Weight {
		(20_263_000 as Weight)
			// Standard Error: 0
			.saturating_add((108_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn remove_sub(s: u32) -> Weight {
		(66_159_000 as Weight)
			// Standard Error: 1_000
			.saturating_add((297_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	fn quit_sub(s: u32) -> Weight {
		(41_567_000 as Weight)
			// Standard Error: 1_000
			.saturating_add((290_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
}
