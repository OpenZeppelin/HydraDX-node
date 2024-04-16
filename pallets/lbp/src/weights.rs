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

//! Autogenerated weights for `pallet_lbp`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2024-04-15, STEPS: `10`, REPEAT: `30`, LOW RANGE: `[]`, HIGH RANGE: `[]`
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
// --wasm-execution=compiled
// --heap-pages=4096
// --template=.maintain/pallet-weight-template-no-back.hbs
// --pallet=pallet-lbp
// --output=weights-1.1.0/lbp.rs
// --extrinsic=*
// --extra

#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(clippy::unnecessary_cast)]

use frame_support::{
	traits::Get,
	weights::{constants::RocksDbWeight, Weight},
};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_lbp.
pub trait WeightInfo {
	fn create_pool() -> Weight;
	fn update_pool_data() -> Weight;
	fn add_liquidity() -> Weight;
	fn remove_liquidity() -> Weight;
	fn sell() -> Weight;
	fn buy() -> Weight;
	fn router_execution_sell(c: u32, e: u32) -> Weight;
	fn router_execution_buy(c: u32, e: u32) -> Weight;
	fn calculate_buy() -> Weight;
	fn calculate_spot_price() -> Weight;
}

/// Weights for pallet_lbp using the hydraDX node and recommended hardware.
pub struct HydraWeight<T>(PhantomData<T>);

impl<T: frame_system::Config> WeightInfo for HydraWeight<T> {
	/// Storage: `LBP::PoolData` (r:1 w:1)
	/// Proof: `LBP::PoolData` (`max_values`: None, `max_size`: Some(163), added: 2638, mode: `MaxEncodedLen`)
	/// Storage: `LBP::FeeCollectorWithAsset` (r:1 w:1)
	/// Proof: `LBP::FeeCollectorWithAsset` (`max_values`: None, `max_size`: Some(69), added: 2544, mode: `MaxEncodedLen`)
	/// Storage: `Tokens::Accounts` (r:4 w:4)
	/// Proof: `Tokens::Accounts` (`max_values`: None, `max_size`: Some(108), added: 2583, mode: `MaxEncodedLen`)
	/// Storage: `Duster::AccountBlacklist` (r:1 w:0)
	/// Proof: `Duster::AccountBlacklist` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `AssetRegistry::BannedAssets` (r:2 w:0)
	/// Proof: `AssetRegistry::BannedAssets` (`max_values`: None, `max_size`: Some(20), added: 2495, mode: `MaxEncodedLen`)
	/// Storage: `AssetRegistry::Assets` (r:2 w:0)
	/// Proof: `AssetRegistry::Assets` (`max_values`: None, `max_size`: Some(125), added: 2600, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:2 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `MultiTransactionPayment::AccountCurrencyMap` (r:1 w:1)
	/// Proof: `MultiTransactionPayment::AccountCurrencyMap` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	/// Storage: `MultiTransactionPayment::AcceptedCurrencies` (r:1 w:0)
	/// Proof: `MultiTransactionPayment::AcceptedCurrencies` (`max_values`: None, `max_size`: Some(28), added: 2503, mode: `MaxEncodedLen`)
	fn create_pool() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1278`
		//  Estimated: `11322`
		// Minimum execution time: 167_421_000 picoseconds.
		Weight::from_parts(168_793_000, 11322)
			.saturating_add(T::DbWeight::get().reads(15))
			.saturating_add(T::DbWeight::get().writes(8))
	}
	/// Storage: `LBP::PoolData` (r:1 w:1)
	/// Proof: `LBP::PoolData` (`max_values`: None, `max_size`: Some(163), added: 2638, mode: `MaxEncodedLen`)
	/// Storage: `LBP::FeeCollectorWithAsset` (r:1 w:2)
	/// Proof: `LBP::FeeCollectorWithAsset` (`max_values`: None, `max_size`: Some(69), added: 2544, mode: `MaxEncodedLen`)
	fn update_pool_data() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `384`
		//  Estimated: `3628`
		// Minimum execution time: 30_510_000 picoseconds.
		Weight::from_parts(30_907_000, 3628)
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: `LBP::PoolData` (r:1 w:0)
	/// Proof: `LBP::PoolData` (`max_values`: None, `max_size`: Some(163), added: 2638, mode: `MaxEncodedLen`)
	/// Storage: `Tokens::Accounts` (r:4 w:4)
	/// Proof: `Tokens::Accounts` (`max_values`: None, `max_size`: Some(108), added: 2583, mode: `MaxEncodedLen`)
	/// Storage: `Duster::AccountBlacklist` (r:1 w:0)
	/// Proof: `Duster::AccountBlacklist` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `AssetRegistry::BannedAssets` (r:2 w:0)
	/// Proof: `AssetRegistry::BannedAssets` (`max_values`: None, `max_size`: Some(20), added: 2495, mode: `MaxEncodedLen`)
	/// Storage: `AssetRegistry::Assets` (r:2 w:0)
	/// Proof: `AssetRegistry::Assets` (`max_values`: None, `max_size`: Some(125), added: 2600, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:0)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	fn add_liquidity() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1539`
		//  Estimated: `11322`
		// Minimum execution time: 121_701_000 picoseconds.
		Weight::from_parts(123_000_000, 11322)
			.saturating_add(T::DbWeight::get().reads(11))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: `LBP::PoolData` (r:1 w:1)
	/// Proof: `LBP::PoolData` (`max_values`: None, `max_size`: Some(163), added: 2638, mode: `MaxEncodedLen`)
	/// Storage: `Tokens::Accounts` (r:4 w:4)
	/// Proof: `Tokens::Accounts` (`max_values`: None, `max_size`: Some(108), added: 2583, mode: `MaxEncodedLen`)
	/// Storage: `Duster::AccountBlacklist` (r:1 w:0)
	/// Proof: `Duster::AccountBlacklist` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `AssetRegistry::BannedAssets` (r:2 w:0)
	/// Proof: `AssetRegistry::BannedAssets` (`max_values`: None, `max_size`: Some(20), added: 2495, mode: `MaxEncodedLen`)
	/// Storage: `AssetRegistry::Assets` (r:2 w:0)
	/// Proof: `AssetRegistry::Assets` (`max_values`: None, `max_size`: Some(125), added: 2600, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `MultiTransactionPayment::AccountCurrencyMap` (r:1 w:1)
	/// Proof: `MultiTransactionPayment::AccountCurrencyMap` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	/// Storage: `Tokens::Locks` (r:1 w:0)
	/// Proof: `Tokens::Locks` (`max_values`: None, `max_size`: Some(1261), added: 3736, mode: `MaxEncodedLen`)
	/// Storage: `LBP::FeeCollectorWithAsset` (r:0 w:1)
	/// Proof: `LBP::FeeCollectorWithAsset` (`max_values`: None, `max_size`: Some(69), added: 2544, mode: `MaxEncodedLen`)
	fn remove_liquidity() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1654`
		//  Estimated: `11322`
		// Minimum execution time: 155_630_000 picoseconds.
		Weight::from_parts(156_726_000, 11322)
			.saturating_add(T::DbWeight::get().reads(13))
			.saturating_add(T::DbWeight::get().writes(8))
	}
	/// Storage: `Tokens::Accounts` (r:5 w:5)
	/// Proof: `Tokens::Accounts` (`max_values`: None, `max_size`: Some(108), added: 2583, mode: `MaxEncodedLen`)
	/// Storage: `LBP::PoolData` (r:1 w:0)
	/// Proof: `LBP::PoolData` (`max_values`: None, `max_size`: Some(163), added: 2638, mode: `MaxEncodedLen`)
	/// Storage: `Tokens::Locks` (r:1 w:1)
	/// Proof: `Tokens::Locks` (`max_values`: None, `max_size`: Some(1261), added: 3736, mode: `MaxEncodedLen`)
	/// Storage: `Duster::AccountBlacklist` (r:2 w:0)
	/// Proof: `Duster::AccountBlacklist` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `AssetRegistry::BannedAssets` (r:2 w:0)
	/// Proof: `AssetRegistry::BannedAssets` (`max_values`: None, `max_size`: Some(20), added: 2495, mode: `MaxEncodedLen`)
	/// Storage: `AssetRegistry::Assets` (r:2 w:0)
	/// Proof: `AssetRegistry::Assets` (`max_values`: None, `max_size`: Some(125), added: 2600, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:3 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	fn sell() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1944`
		//  Estimated: `13905`
		// Minimum execution time: 245_392_000 picoseconds.
		Weight::from_parts(247_320_000, 13905)
			.saturating_add(T::DbWeight::get().reads(16))
			.saturating_add(T::DbWeight::get().writes(7))
	}
	/// Storage: `LBP::PoolData` (r:1 w:0)
	/// Proof: `LBP::PoolData` (`max_values`: None, `max_size`: Some(163), added: 2638, mode: `MaxEncodedLen`)
	/// Storage: `Tokens::Accounts` (r:5 w:5)
	/// Proof: `Tokens::Accounts` (`max_values`: None, `max_size`: Some(108), added: 2583, mode: `MaxEncodedLen`)
	/// Storage: `Tokens::Locks` (r:1 w:1)
	/// Proof: `Tokens::Locks` (`max_values`: None, `max_size`: Some(1261), added: 3736, mode: `MaxEncodedLen`)
	/// Storage: `Duster::AccountBlacklist` (r:2 w:0)
	/// Proof: `Duster::AccountBlacklist` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `AssetRegistry::BannedAssets` (r:2 w:0)
	/// Proof: `AssetRegistry::BannedAssets` (`max_values`: None, `max_size`: Some(20), added: 2495, mode: `MaxEncodedLen`)
	/// Storage: `AssetRegistry::Assets` (r:2 w:0)
	/// Proof: `AssetRegistry::Assets` (`max_values`: None, `max_size`: Some(125), added: 2600, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:3 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	fn buy() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1944`
		//  Estimated: `13905`
		// Minimum execution time: 247_168_000 picoseconds.
		Weight::from_parts(248_543_000, 13905)
			.saturating_add(T::DbWeight::get().reads(16))
			.saturating_add(T::DbWeight::get().writes(7))
	}
	/// Storage: `LBP::PoolData` (r:1 w:0)
	/// Proof: `LBP::PoolData` (`max_values`: None, `max_size`: Some(163), added: 2638, mode: `MaxEncodedLen`)
	/// Storage: `Tokens::Accounts` (r:5 w:5)
	/// Proof: `Tokens::Accounts` (`max_values`: None, `max_size`: Some(108), added: 2583, mode: `MaxEncodedLen`)
	/// Storage: `Tokens::Locks` (r:1 w:1)
	/// Proof: `Tokens::Locks` (`max_values`: None, `max_size`: Some(1261), added: 3736, mode: `MaxEncodedLen`)
	/// Storage: `Duster::AccountBlacklist` (r:2 w:0)
	/// Proof: `Duster::AccountBlacklist` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `AssetRegistry::BannedAssets` (r:2 w:0)
	/// Proof: `AssetRegistry::BannedAssets` (`max_values`: None, `max_size`: Some(20), added: 2495, mode: `MaxEncodedLen`)
	/// Storage: `AssetRegistry::Assets` (r:2 w:0)
	/// Proof: `AssetRegistry::Assets` (`max_values`: None, `max_size`: Some(125), added: 2600, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:3 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// The range of component `c` is `[1, 2]`.
	/// The range of component `e` is `[0, 1]`.
	fn router_execution_sell(c: u32, e: u32) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `789 + e * (1155 ±0)`
		//  Estimated: `6156 + e * (7749 ±245_709_589_663_843_264)`
		// Minimum execution time: 66_501_000 picoseconds.
		Weight::from_parts(67_000_000, 6156)
			// Standard Error: 582_591
			.saturating_add(Weight::from_parts(2_266_784, 0).saturating_mul(c.into()))
			// Standard Error: 1_278_955
			.saturating_add(Weight::from_parts(186_170_573, 0).saturating_mul(e.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().reads((13_u64).saturating_mul(e.into())))
			.saturating_add(T::DbWeight::get().writes((7_u64).saturating_mul(e.into())))
			.saturating_add(Weight::from_parts(0, 7749).saturating_mul(e.into()))
	}
	/// Storage: `LBP::PoolData` (r:1 w:0)
	/// Proof: `LBP::PoolData` (`max_values`: None, `max_size`: Some(163), added: 2638, mode: `MaxEncodedLen`)
	/// Storage: `Tokens::Accounts` (r:5 w:5)
	/// Proof: `Tokens::Accounts` (`max_values`: None, `max_size`: Some(108), added: 2583, mode: `MaxEncodedLen`)
	/// Storage: `Tokens::Locks` (r:1 w:1)
	/// Proof: `Tokens::Locks` (`max_values`: None, `max_size`: Some(1261), added: 3736, mode: `MaxEncodedLen`)
	/// Storage: `Duster::AccountBlacklist` (r:2 w:0)
	/// Proof: `Duster::AccountBlacklist` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `AssetRegistry::BannedAssets` (r:2 w:0)
	/// Proof: `AssetRegistry::BannedAssets` (`max_values`: None, `max_size`: Some(20), added: 2495, mode: `MaxEncodedLen`)
	/// Storage: `AssetRegistry::Assets` (r:2 w:0)
	/// Proof: `AssetRegistry::Assets` (`max_values`: None, `max_size`: Some(125), added: 2600, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:3 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// The range of component `c` is `[1, 3]`.
	/// The range of component `e` is `[0, 1]`.
	fn router_execution_buy(c: u32, e: u32) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `789 + e * (1155 ±0)`
		//  Estimated: `6156 + e * (7749 ±0)`
		// Minimum execution time: 117_257_000 picoseconds.
		Weight::from_parts(118_182_000, 6156)
			// Standard Error: 747_210
			.saturating_add(Weight::from_parts(3_788_250, 0).saturating_mul(c.into()))
			// Standard Error: 2_467_334
			.saturating_add(Weight::from_parts(157_907_441, 0).saturating_mul(e.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().reads((13_u64).saturating_mul(e.into())))
			.saturating_add(T::DbWeight::get().writes((7_u64).saturating_mul(e.into())))
			.saturating_add(Weight::from_parts(0, 7749).saturating_mul(e.into()))
	}
	/// Storage: `LBP::PoolData` (r:1 w:0)
	/// Proof: `LBP::PoolData` (`max_values`: None, `max_size`: Some(163), added: 2638, mode: `MaxEncodedLen`)
	/// Storage: `Tokens::Accounts` (r:2 w:0)
	/// Proof: `Tokens::Accounts` (`max_values`: None, `max_size`: Some(108), added: 2583, mode: `MaxEncodedLen`)
	fn calculate_buy() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `789`
		//  Estimated: `6156`
		// Minimum execution time: 67_298_000 picoseconds.
		Weight::from_parts(68_051_000, 6156).saturating_add(T::DbWeight::get().reads(3))
	}
	/// Storage: `LBP::PoolData` (r:1 w:0)
	/// Proof: `LBP::PoolData` (`max_values`: None, `max_size`: Some(163), added: 2638, mode: `MaxEncodedLen`)
	/// Storage: `Tokens::Accounts` (r:2 w:0)
	/// Proof: `Tokens::Accounts` (`max_values`: None, `max_size`: Some(108), added: 2583, mode: `MaxEncodedLen`)
	/// Storage: `Tokens::Locks` (r:1 w:0)
	/// Proof: `Tokens::Locks` (`max_values`: None, `max_size`: Some(1261), added: 3736, mode: `MaxEncodedLen`)
	fn calculate_spot_price() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `789`
		//  Estimated: `6156`
		// Minimum execution time: 33_145_000 picoseconds.
		Weight::from_parts(33_774_000, 6156).saturating_add(T::DbWeight::get().reads(4))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	/// Storage: `LBP::PoolData` (r:1 w:1)
	/// Proof: `LBP::PoolData` (`max_values`: None, `max_size`: Some(163), added: 2638, mode: `MaxEncodedLen`)
	/// Storage: `LBP::FeeCollectorWithAsset` (r:1 w:1)
	/// Proof: `LBP::FeeCollectorWithAsset` (`max_values`: None, `max_size`: Some(69), added: 2544, mode: `MaxEncodedLen`)
	/// Storage: `Tokens::Accounts` (r:4 w:4)
	/// Proof: `Tokens::Accounts` (`max_values`: None, `max_size`: Some(108), added: 2583, mode: `MaxEncodedLen`)
	/// Storage: `Duster::AccountBlacklist` (r:1 w:0)
	/// Proof: `Duster::AccountBlacklist` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `AssetRegistry::BannedAssets` (r:2 w:0)
	/// Proof: `AssetRegistry::BannedAssets` (`max_values`: None, `max_size`: Some(20), added: 2495, mode: `MaxEncodedLen`)
	/// Storage: `AssetRegistry::Assets` (r:2 w:0)
	/// Proof: `AssetRegistry::Assets` (`max_values`: None, `max_size`: Some(125), added: 2600, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:2 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `MultiTransactionPayment::AccountCurrencyMap` (r:1 w:1)
	/// Proof: `MultiTransactionPayment::AccountCurrencyMap` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	/// Storage: `MultiTransactionPayment::AcceptedCurrencies` (r:1 w:0)
	/// Proof: `MultiTransactionPayment::AcceptedCurrencies` (`max_values`: None, `max_size`: Some(28), added: 2503, mode: `MaxEncodedLen`)
	fn create_pool() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1278`
		//  Estimated: `11322`
		// Minimum execution time: 167_421_000 picoseconds.
		Weight::from_parts(168_793_000, 11322)
			.saturating_add(RocksDbWeight::get().reads(15))
			.saturating_add(RocksDbWeight::get().writes(8))
	}
	/// Storage: `LBP::PoolData` (r:1 w:1)
	/// Proof: `LBP::PoolData` (`max_values`: None, `max_size`: Some(163), added: 2638, mode: `MaxEncodedLen`)
	/// Storage: `LBP::FeeCollectorWithAsset` (r:1 w:2)
	/// Proof: `LBP::FeeCollectorWithAsset` (`max_values`: None, `max_size`: Some(69), added: 2544, mode: `MaxEncodedLen`)
	fn update_pool_data() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `384`
		//  Estimated: `3628`
		// Minimum execution time: 30_510_000 picoseconds.
		Weight::from_parts(30_907_000, 3628)
			.saturating_add(RocksDbWeight::get().reads(2))
			.saturating_add(RocksDbWeight::get().writes(3))
	}
	/// Storage: `LBP::PoolData` (r:1 w:0)
	/// Proof: `LBP::PoolData` (`max_values`: None, `max_size`: Some(163), added: 2638, mode: `MaxEncodedLen`)
	/// Storage: `Tokens::Accounts` (r:4 w:4)
	/// Proof: `Tokens::Accounts` (`max_values`: None, `max_size`: Some(108), added: 2583, mode: `MaxEncodedLen`)
	/// Storage: `Duster::AccountBlacklist` (r:1 w:0)
	/// Proof: `Duster::AccountBlacklist` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `AssetRegistry::BannedAssets` (r:2 w:0)
	/// Proof: `AssetRegistry::BannedAssets` (`max_values`: None, `max_size`: Some(20), added: 2495, mode: `MaxEncodedLen`)
	/// Storage: `AssetRegistry::Assets` (r:2 w:0)
	/// Proof: `AssetRegistry::Assets` (`max_values`: None, `max_size`: Some(125), added: 2600, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:0)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	fn add_liquidity() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1539`
		//  Estimated: `11322`
		// Minimum execution time: 121_701_000 picoseconds.
		Weight::from_parts(123_000_000, 11322)
			.saturating_add(RocksDbWeight::get().reads(11))
			.saturating_add(RocksDbWeight::get().writes(4))
	}
	/// Storage: `LBP::PoolData` (r:1 w:1)
	/// Proof: `LBP::PoolData` (`max_values`: None, `max_size`: Some(163), added: 2638, mode: `MaxEncodedLen`)
	/// Storage: `Tokens::Accounts` (r:4 w:4)
	/// Proof: `Tokens::Accounts` (`max_values`: None, `max_size`: Some(108), added: 2583, mode: `MaxEncodedLen`)
	/// Storage: `Duster::AccountBlacklist` (r:1 w:0)
	/// Proof: `Duster::AccountBlacklist` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `AssetRegistry::BannedAssets` (r:2 w:0)
	/// Proof: `AssetRegistry::BannedAssets` (`max_values`: None, `max_size`: Some(20), added: 2495, mode: `MaxEncodedLen`)
	/// Storage: `AssetRegistry::Assets` (r:2 w:0)
	/// Proof: `AssetRegistry::Assets` (`max_values`: None, `max_size`: Some(125), added: 2600, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `MultiTransactionPayment::AccountCurrencyMap` (r:1 w:1)
	/// Proof: `MultiTransactionPayment::AccountCurrencyMap` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	/// Storage: `Tokens::Locks` (r:1 w:0)
	/// Proof: `Tokens::Locks` (`max_values`: None, `max_size`: Some(1261), added: 3736, mode: `MaxEncodedLen`)
	/// Storage: `LBP::FeeCollectorWithAsset` (r:0 w:1)
	/// Proof: `LBP::FeeCollectorWithAsset` (`max_values`: None, `max_size`: Some(69), added: 2544, mode: `MaxEncodedLen`)
	fn remove_liquidity() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1654`
		//  Estimated: `11322`
		// Minimum execution time: 155_630_000 picoseconds.
		Weight::from_parts(156_726_000, 11322)
			.saturating_add(RocksDbWeight::get().reads(13))
			.saturating_add(RocksDbWeight::get().writes(8))
	}
	/// Storage: `Tokens::Accounts` (r:5 w:5)
	/// Proof: `Tokens::Accounts` (`max_values`: None, `max_size`: Some(108), added: 2583, mode: `MaxEncodedLen`)
	/// Storage: `LBP::PoolData` (r:1 w:0)
	/// Proof: `LBP::PoolData` (`max_values`: None, `max_size`: Some(163), added: 2638, mode: `MaxEncodedLen`)
	/// Storage: `Tokens::Locks` (r:1 w:1)
	/// Proof: `Tokens::Locks` (`max_values`: None, `max_size`: Some(1261), added: 3736, mode: `MaxEncodedLen`)
	/// Storage: `Duster::AccountBlacklist` (r:2 w:0)
	/// Proof: `Duster::AccountBlacklist` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `AssetRegistry::BannedAssets` (r:2 w:0)
	/// Proof: `AssetRegistry::BannedAssets` (`max_values`: None, `max_size`: Some(20), added: 2495, mode: `MaxEncodedLen`)
	/// Storage: `AssetRegistry::Assets` (r:2 w:0)
	/// Proof: `AssetRegistry::Assets` (`max_values`: None, `max_size`: Some(125), added: 2600, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:3 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	fn sell() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1944`
		//  Estimated: `13905`
		// Minimum execution time: 245_392_000 picoseconds.
		Weight::from_parts(247_320_000, 13905)
			.saturating_add(RocksDbWeight::get().reads(16))
			.saturating_add(RocksDbWeight::get().writes(7))
	}
	/// Storage: `LBP::PoolData` (r:1 w:0)
	/// Proof: `LBP::PoolData` (`max_values`: None, `max_size`: Some(163), added: 2638, mode: `MaxEncodedLen`)
	/// Storage: `Tokens::Accounts` (r:5 w:5)
	/// Proof: `Tokens::Accounts` (`max_values`: None, `max_size`: Some(108), added: 2583, mode: `MaxEncodedLen`)
	/// Storage: `Tokens::Locks` (r:1 w:1)
	/// Proof: `Tokens::Locks` (`max_values`: None, `max_size`: Some(1261), added: 3736, mode: `MaxEncodedLen`)
	/// Storage: `Duster::AccountBlacklist` (r:2 w:0)
	/// Proof: `Duster::AccountBlacklist` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `AssetRegistry::BannedAssets` (r:2 w:0)
	/// Proof: `AssetRegistry::BannedAssets` (`max_values`: None, `max_size`: Some(20), added: 2495, mode: `MaxEncodedLen`)
	/// Storage: `AssetRegistry::Assets` (r:2 w:0)
	/// Proof: `AssetRegistry::Assets` (`max_values`: None, `max_size`: Some(125), added: 2600, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:3 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	fn buy() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1944`
		//  Estimated: `13905`
		// Minimum execution time: 247_168_000 picoseconds.
		Weight::from_parts(248_543_000, 13905)
			.saturating_add(RocksDbWeight::get().reads(16))
			.saturating_add(RocksDbWeight::get().writes(7))
	}
	/// Storage: `LBP::PoolData` (r:1 w:0)
	/// Proof: `LBP::PoolData` (`max_values`: None, `max_size`: Some(163), added: 2638, mode: `MaxEncodedLen`)
	/// Storage: `Tokens::Accounts` (r:5 w:5)
	/// Proof: `Tokens::Accounts` (`max_values`: None, `max_size`: Some(108), added: 2583, mode: `MaxEncodedLen`)
	/// Storage: `Tokens::Locks` (r:1 w:1)
	/// Proof: `Tokens::Locks` (`max_values`: None, `max_size`: Some(1261), added: 3736, mode: `MaxEncodedLen`)
	/// Storage: `Duster::AccountBlacklist` (r:2 w:0)
	/// Proof: `Duster::AccountBlacklist` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `AssetRegistry::BannedAssets` (r:2 w:0)
	/// Proof: `AssetRegistry::BannedAssets` (`max_values`: None, `max_size`: Some(20), added: 2495, mode: `MaxEncodedLen`)
	/// Storage: `AssetRegistry::Assets` (r:2 w:0)
	/// Proof: `AssetRegistry::Assets` (`max_values`: None, `max_size`: Some(125), added: 2600, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:3 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// The range of component `c` is `[1, 2]`.
	/// The range of component `e` is `[0, 1]`.
	fn router_execution_sell(c: u32, e: u32) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `789 + e * (1155 ±0)`
		//  Estimated: `6156 + e * (7749 ±245_709_589_663_843_264)`
		// Minimum execution time: 66_501_000 picoseconds.
		Weight::from_parts(67_000_000, 6156)
			// Standard Error: 582_591
			.saturating_add(Weight::from_parts(2_266_784, 0).saturating_mul(c.into()))
			// Standard Error: 1_278_955
			.saturating_add(Weight::from_parts(186_170_573, 0).saturating_mul(e.into()))
			.saturating_add(RocksDbWeight::get().reads(3))
			.saturating_add(RocksDbWeight::get().reads((13_u64).saturating_mul(e.into())))
			.saturating_add(RocksDbWeight::get().writes((7_u64).saturating_mul(e.into())))
			.saturating_add(Weight::from_parts(0, 7749).saturating_mul(e.into()))
	}
	/// Storage: `LBP::PoolData` (r:1 w:0)
	/// Proof: `LBP::PoolData` (`max_values`: None, `max_size`: Some(163), added: 2638, mode: `MaxEncodedLen`)
	/// Storage: `Tokens::Accounts` (r:5 w:5)
	/// Proof: `Tokens::Accounts` (`max_values`: None, `max_size`: Some(108), added: 2583, mode: `MaxEncodedLen`)
	/// Storage: `Tokens::Locks` (r:1 w:1)
	/// Proof: `Tokens::Locks` (`max_values`: None, `max_size`: Some(1261), added: 3736, mode: `MaxEncodedLen`)
	/// Storage: `Duster::AccountBlacklist` (r:2 w:0)
	/// Proof: `Duster::AccountBlacklist` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `AssetRegistry::BannedAssets` (r:2 w:0)
	/// Proof: `AssetRegistry::BannedAssets` (`max_values`: None, `max_size`: Some(20), added: 2495, mode: `MaxEncodedLen`)
	/// Storage: `AssetRegistry::Assets` (r:2 w:0)
	/// Proof: `AssetRegistry::Assets` (`max_values`: None, `max_size`: Some(125), added: 2600, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:3 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// The range of component `c` is `[1, 3]`.
	/// The range of component `e` is `[0, 1]`.
	fn router_execution_buy(c: u32, e: u32) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `789 + e * (1155 ±0)`
		//  Estimated: `6156 + e * (7749 ±0)`
		// Minimum execution time: 117_257_000 picoseconds.
		Weight::from_parts(118_182_000, 6156)
			// Standard Error: 747_210
			.saturating_add(Weight::from_parts(3_788_250, 0).saturating_mul(c.into()))
			// Standard Error: 2_467_334
			.saturating_add(Weight::from_parts(157_907_441, 0).saturating_mul(e.into()))
			.saturating_add(RocksDbWeight::get().reads(3))
			.saturating_add(RocksDbWeight::get().reads((13_u64).saturating_mul(e.into())))
			.saturating_add(RocksDbWeight::get().writes((7_u64).saturating_mul(e.into())))
			.saturating_add(Weight::from_parts(0, 7749).saturating_mul(e.into()))
	}
	/// Storage: `LBP::PoolData` (r:1 w:0)
	/// Proof: `LBP::PoolData` (`max_values`: None, `max_size`: Some(163), added: 2638, mode: `MaxEncodedLen`)
	/// Storage: `Tokens::Accounts` (r:2 w:0)
	/// Proof: `Tokens::Accounts` (`max_values`: None, `max_size`: Some(108), added: 2583, mode: `MaxEncodedLen`)
	fn calculate_buy() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `789`
		//  Estimated: `6156`
		// Minimum execution time: 67_298_000 picoseconds.
		Weight::from_parts(68_051_000, 6156).saturating_add(RocksDbWeight::get().reads(3))
	}
	/// Storage: `LBP::PoolData` (r:1 w:0)
	/// Proof: `LBP::PoolData` (`max_values`: None, `max_size`: Some(163), added: 2638, mode: `MaxEncodedLen`)
	/// Storage: `Tokens::Accounts` (r:2 w:0)
	/// Proof: `Tokens::Accounts` (`max_values`: None, `max_size`: Some(108), added: 2583, mode: `MaxEncodedLen`)
	/// Storage: `Tokens::Locks` (r:1 w:0)
	/// Proof: `Tokens::Locks` (`max_values`: None, `max_size`: Some(1261), added: 3736, mode: `MaxEncodedLen`)
	fn calculate_spot_price() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `789`
		//  Estimated: `6156`
		// Minimum execution time: 33_145_000 picoseconds.
		Weight::from_parts(33_774_000, 6156).saturating_add(RocksDbWeight::get().reads(4))
	}
}
