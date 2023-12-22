// This file is part of HydraDX.

// Copyright (C) 2020-2023  Intergalactic, Limited (GIB).
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Autogenerated weights for `cumulus_pallet_xcmp_queue`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-12-14, STEPS: `10`, REPEAT: `30`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `bench-bot`, CPU: `Intel(R) Core(TM) i7-7700K CPU @ 4.20GHz`
//! WASM-EXECUTION: `Compiled`, CHAIN: `Some("dev")`, DB CACHE: 1024

// Executed Command:
// target/release/hydradx
// benchmark
// pallet
// --chain=dev
// --steps=10
// --repeat=30
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --template=.maintain/pallet-weight-template-no-back.hbs
// --pallet=cumulus-pallet-xcmp-queue
// --output=weights-1.1.0/xcmp-queue.rs
// --extrinsic=

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `cumulus_pallet_xcmp_queue`.
pub struct HydraWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> cumulus_pallet_xcmp_queue::WeightInfo for HydraWeight<T> {
	/// Storage: `XcmpQueue::QueueConfig` (r:1 w:1)
	/// Proof: `XcmpQueue::QueueConfig` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	fn set_config_with_u32() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `142`
		//  Estimated: `1627`
		// Minimum execution time: 8_061_000 picoseconds.
		Weight::from_parts(8_243_000, 1627)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `XcmpQueue::QueueConfig` (r:1 w:1)
	/// Proof: `XcmpQueue::QueueConfig` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	fn set_config_with_weight() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `142`
		//  Estimated: `1627`
		// Minimum execution time: 8_255_000 picoseconds.
		Weight::from_parts(8_413_000, 1627)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `XcmpQueue::QueueConfig` (r:1 w:0)
	/// Proof: `XcmpQueue::QueueConfig` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `XcmpQueue::QueueSuspended` (r:1 w:0)
	/// Proof: `XcmpQueue::QueueSuspended` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `XcmpQueue::DeferredQueueSuspended` (r:1 w:0)
	/// Proof: `XcmpQueue::DeferredQueueSuspended` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `XcmpQueue::DeferredIndices` (r:1 w:1)
	/// Proof: `XcmpQueue::DeferredIndices` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `XcmpQueue::DeferredMessageBuckets` (r:3 w:3)
	/// Proof: `XcmpQueue::DeferredMessageBuckets` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `XcmpQueue::CounterForOverweight` (r:1 w:1)
	/// Proof: `XcmpQueue::CounterForOverweight` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `XcmpQueue::OverweightCount` (r:1 w:1)
	/// Proof: `XcmpQueue::OverweightCount` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `XcmpQueue::Overweight` (r:60 w:60)
	/// Proof: `XcmpQueue::Overweight` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `b` is `[1, 3]`.
	fn service_deferred(b: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `6275 + b * (324357 ±0)`
		//  Estimated: `9740 + b * (373857 ±0)`
		// Minimum execution time: 32_529_153_000 picoseconds.
		Weight::from_parts(160_904_784, 9740)
			// Standard Error: 8_949_581
			.saturating_add(Weight::from_parts(32_575_018_568, 0).saturating_mul(b.into()))
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().reads((21_u64).saturating_mul(b.into())))
			.saturating_add(T::DbWeight::get().writes(3))
			.saturating_add(T::DbWeight::get().writes((21_u64).saturating_mul(b.into())))
			.saturating_add(Weight::from_parts(0, 373857).saturating_mul(b.into()))
	}
	/// Storage: `XcmpQueue::DeferredMessageBuckets` (r:1 w:1)
	/// Proof: `XcmpQueue::DeferredMessageBuckets` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `m` is `[1, 20]`.
	fn discard_deferred_bucket(m: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `252 + m * (16216 ±0)`
		//  Estimated: `3716 + m * (16216 ±0)`
		// Minimum execution time: 1_269_514_000 picoseconds.
		Weight::from_parts(254_832_812, 3716)
			// Standard Error: 451_448
			.saturating_add(Weight::from_parts(1_075_219_465, 0).saturating_mul(m.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
			.saturating_add(Weight::from_parts(0, 16216).saturating_mul(m.into()))
	}
	/// Storage: `XcmpQueue::DeferredMessageBuckets` (r:1 w:1)
	/// Proof: `XcmpQueue::DeferredMessageBuckets` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `m` is `[1, 20]`.
	fn discard_deferred_individual(m: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `252 + m * (16216 ±0)`
		//  Estimated: `3716 + m * (16216 ±0)`
		// Minimum execution time: 1_331_675_000 picoseconds.
		Weight::from_parts(131_637_639, 3716)
			// Standard Error: 513_067
			.saturating_add(Weight::from_parts(1_197_225_867, 0).saturating_mul(m.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
			.saturating_add(Weight::from_parts(0, 16216).saturating_mul(m.into()))
	}
	/// Storage: `XcmpQueue::DeferredIndices` (r:1 w:1)
	/// Proof: `XcmpQueue::DeferredIndices` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `XcmpQueue::DeferredMessageBuckets` (r:1 w:1)
	/// Proof: `XcmpQueue::DeferredMessageBuckets` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `m` is `[1, 20]`.
	fn try_place_in_deferred_queue(m: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0 + m * (16216 ±0)`
		//  Estimated: `9724 + m * (15020 ±55)`
		// Minimum execution time: 106_584_000 picoseconds.
		Weight::from_parts(125_484_886, 9724)
			// Standard Error: 199_725
			.saturating_add(Weight::from_parts(6_590_187, 0).saturating_mul(m.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
			.saturating_add(Weight::from_parts(0, 15020).saturating_mul(m.into()))
	}
	// Storage: XcmpQueue DeferredMessageBuckets (r:1 w:1)
	// Proof Skipped: XcmpQueue DeferredMessageBuckets (max_values: None, max_size: None, mode: Measured)
	fn discard_deferred_individual() -> Weight {
		// Minimum execution time: 124_783_889 nanoseconds.
		Weight::from_ref_time(125_149_821_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: XcmpQueue DeferredIndices (r:1 w:1)
	// Proof Skipped: XcmpQueue DeferredIndices (max_values: None, max_size: None, mode: Measured)
	// Storage: XcmpQueue DeferredMessageBuckets (r:1 w:1)
	// Proof Skipped: XcmpQueue DeferredMessageBuckets (max_values: None, max_size: None, mode: Measured)
	fn try_place_in_deferred_queue() -> Weight {
		// Minimum execution time: 530_788 nanoseconds.
		Weight::from_ref_time(537_065_000 as u64)
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
}
