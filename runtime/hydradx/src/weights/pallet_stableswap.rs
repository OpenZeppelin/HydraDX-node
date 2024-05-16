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

//! Autogenerated weights for `pallet_stableswap`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2024-02-15, STEPS: `10`, REPEAT: `30`, LOW RANGE: `[]`, HIGH RANGE: `[]`
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
// --pallet=pallet_stableswap
// --output=./weights/stableswap.rs
// --extrinsic=*

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

use pallet_stableswap::weights::WeightInfo;

/// Weight functions for `pallet_stableswap`.
pub struct HydraWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for HydraWeight<T> {
	/// Storage: `Stableswap::Pools` (r:1 w:1)
	/// Proof: `Stableswap::Pools` (`max_values`: None, `max_size`: Some(57), added: 2532, mode: `MaxEncodedLen`)
	/// Storage: `AssetRegistry::Assets` (r:6 w:0)
	/// Proof: `AssetRegistry::Assets` (`max_values`: None, `max_size`: Some(125), added: 2600, mode: `MaxEncodedLen`)
	/// Storage: `Duster::AccountBlacklist` (r:0 w:1)
	/// Proof: `Duster::AccountBlacklist` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	fn create_pool() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `789`
		//  Estimated: `16590`
		// Minimum execution time: 53_155_000 picoseconds.
		Weight::from_parts(53_722_000, 16590)
			.saturating_add(T::DbWeight::get().reads(7))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `Stableswap::Pools` (r:1 w:0)
	/// Proof: `Stableswap::Pools` (`max_values`: None, `max_size`: Some(57), added: 2532, mode: `MaxEncodedLen`)
	/// Storage: `Stableswap::AssetTradability` (r:5 w:0)
	/// Proof: `Stableswap::AssetTradability` (`max_values`: None, `max_size`: Some(41), added: 2516, mode: `MaxEncodedLen`)
	/// Storage: `Tokens::Accounts` (r:11 w:11)
	/// Proof: `Tokens::Accounts` (`max_values`: None, `max_size`: Some(108), added: 2583, mode: `MaxEncodedLen`)
	/// Storage: `AssetRegistry::Assets` (r:6 w:0)
	/// Proof: `AssetRegistry::Assets` (`max_values`: None, `max_size`: Some(125), added: 2600, mode: `MaxEncodedLen`)
	/// Storage: `Tokens::TotalIssuance` (r:1 w:1)
	/// Proof: `Tokens::TotalIssuance` (`max_values`: None, `max_size`: Some(28), added: 2503, mode: `MaxEncodedLen`)
	/// Storage: `AssetRegistry::BannedAssets` (r:6 w:0)
	/// Proof: `AssetRegistry::BannedAssets` (`max_values`: None, `max_size`: Some(20), added: 2495, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `MultiTransactionPayment::AccountCurrencyMap` (r:1 w:0)
	/// Proof: `MultiTransactionPayment::AccountCurrencyMap` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	/// Storage: `MultiTransactionPayment::AcceptedCurrencies` (r:1 w:0)
	/// Proof: `MultiTransactionPayment::AcceptedCurrencies` (`max_values`: None, `max_size`: Some(28), added: 2503, mode: `MaxEncodedLen`)
	/// Storage: `Duster::AccountBlacklist` (r:1 w:0)
	/// Proof: `Duster::AccountBlacklist` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `EmaOracle::Accumulator` (r:1 w:1)
	/// Proof: `EmaOracle::Accumulator` (`max_values`: Some(1), `max_size`: Some(5921), added: 6416, mode: `MaxEncodedLen`)
	fn add_liquidity() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3339`
		//  Estimated: `29403`
		// Minimum execution time: 1_155_933_000 picoseconds.
		Weight::from_parts(1_159_202_000, 29403)
			.saturating_add(T::DbWeight::get().reads(35))
			.saturating_add(T::DbWeight::get().writes(14))
	}
	/// Storage: `Stableswap::AssetTradability` (r:1 w:0)
	/// Proof: `Stableswap::AssetTradability` (`max_values`: None, `max_size`: Some(41), added: 2516, mode: `MaxEncodedLen`)
	/// Storage: `Stableswap::Pools` (r:1 w:0)
	/// Proof: `Stableswap::Pools` (`max_values`: None, `max_size`: Some(57), added: 2532, mode: `MaxEncodedLen`)
	/// Storage: `Tokens::TotalIssuance` (r:1 w:1)
	/// Proof: `Tokens::TotalIssuance` (`max_values`: None, `max_size`: Some(28), added: 2503, mode: `MaxEncodedLen`)
	/// Storage: `Tokens::Accounts` (r:7 w:3)
	/// Proof: `Tokens::Accounts` (`max_values`: None, `max_size`: Some(108), added: 2583, mode: `MaxEncodedLen`)
	/// Storage: `AssetRegistry::Assets` (r:6 w:0)
	/// Proof: `AssetRegistry::Assets` (`max_values`: None, `max_size`: Some(125), added: 2600, mode: `MaxEncodedLen`)
	/// Storage: `AssetRegistry::BannedAssets` (r:2 w:0)
	/// Proof: `AssetRegistry::BannedAssets` (`max_values`: None, `max_size`: Some(20), added: 2495, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `MultiTransactionPayment::AccountCurrencyMap` (r:1 w:0)
	/// Proof: `MultiTransactionPayment::AccountCurrencyMap` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	/// Storage: `MultiTransactionPayment::AcceptedCurrencies` (r:1 w:0)
	/// Proof: `MultiTransactionPayment::AcceptedCurrencies` (`max_values`: None, `max_size`: Some(28), added: 2503, mode: `MaxEncodedLen`)
	/// Storage: `Duster::AccountBlacklist` (r:1 w:0)
	/// Proof: `Duster::AccountBlacklist` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `EmaOracle::Accumulator` (r:1 w:1)
	/// Proof: `EmaOracle::Accumulator` (`max_values`: Some(1), `max_size`: Some(5921), added: 6416, mode: `MaxEncodedLen`)
	fn add_liquidity_shares() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3207`
		//  Estimated: `19071`
		// Minimum execution time: 775_198_000 picoseconds.
		Weight::from_parts(779_737_000, 19071)
			.saturating_add(T::DbWeight::get().reads(23))
			.saturating_add(T::DbWeight::get().writes(6))
	}
	/// Storage: `Stableswap::AssetTradability` (r:1 w:0)
	/// Proof: `Stableswap::AssetTradability` (`max_values`: None, `max_size`: Some(41), added: 2516, mode: `MaxEncodedLen`)
	/// Storage: `Tokens::Accounts` (r:7 w:3)
	/// Proof: `Tokens::Accounts` (`max_values`: None, `max_size`: Some(108), added: 2583, mode: `MaxEncodedLen`)
	/// Storage: `Stableswap::Pools` (r:1 w:0)
	/// Proof: `Stableswap::Pools` (`max_values`: None, `max_size`: Some(57), added: 2532, mode: `MaxEncodedLen`)
	/// Storage: `AssetRegistry::Assets` (r:6 w:0)
	/// Proof: `AssetRegistry::Assets` (`max_values`: None, `max_size`: Some(125), added: 2600, mode: `MaxEncodedLen`)
	/// Storage: `Tokens::TotalIssuance` (r:1 w:1)
	/// Proof: `Tokens::TotalIssuance` (`max_values`: None, `max_size`: Some(28), added: 2503, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:2 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Duster::AccountBlacklist` (r:1 w:0)
	/// Proof: `Duster::AccountBlacklist` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `AssetRegistry::BannedAssets` (r:1 w:0)
	/// Proof: `AssetRegistry::BannedAssets` (`max_values`: None, `max_size`: Some(20), added: 2495, mode: `MaxEncodedLen`)
	/// Storage: `MultiTransactionPayment::AcceptedCurrencies` (r:1 w:0)
	/// Proof: `MultiTransactionPayment::AcceptedCurrencies` (`max_values`: None, `max_size`: Some(28), added: 2503, mode: `MaxEncodedLen`)
	/// Storage: `EmaOracle::Accumulator` (r:1 w:1)
	/// Proof: `EmaOracle::Accumulator` (`max_values`: Some(1), `max_size`: Some(5921), added: 6416, mode: `MaxEncodedLen`)
	/// Storage: `MultiTransactionPayment::AccountCurrencyMap` (r:0 w:1)
	/// Proof: `MultiTransactionPayment::AccountCurrencyMap` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	fn remove_liquidity_one_asset() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3231`
		//  Estimated: `19071`
		// Minimum execution time: 807_519_000 picoseconds.
		Weight::from_parts(811_614_000, 19071)
			.saturating_add(T::DbWeight::get().reads(22))
			.saturating_add(T::DbWeight::get().writes(7))
	}
	/// Storage: `Stableswap::AssetTradability` (r:1 w:0)
	/// Proof: `Stableswap::AssetTradability` (`max_values`: None, `max_size`: Some(41), added: 2516, mode: `MaxEncodedLen`)
	/// Storage: `Stableswap::Pools` (r:1 w:0)
	/// Proof: `Stableswap::Pools` (`max_values`: None, `max_size`: Some(57), added: 2532, mode: `MaxEncodedLen`)
	/// Storage: `Tokens::Accounts` (r:7 w:3)
	/// Proof: `Tokens::Accounts` (`max_values`: None, `max_size`: Some(108), added: 2583, mode: `MaxEncodedLen`)
	/// Storage: `AssetRegistry::Assets` (r:6 w:0)
	/// Proof: `AssetRegistry::Assets` (`max_values`: None, `max_size`: Some(125), added: 2600, mode: `MaxEncodedLen`)
	/// Storage: `Tokens::TotalIssuance` (r:1 w:1)
	/// Proof: `Tokens::TotalIssuance` (`max_values`: None, `max_size`: Some(28), added: 2503, mode: `MaxEncodedLen`)
	/// Storage: `Duster::AccountBlacklist` (r:1 w:0)
	/// Proof: `Duster::AccountBlacklist` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `AssetRegistry::BannedAssets` (r:1 w:0)
	/// Proof: `AssetRegistry::BannedAssets` (`max_values`: None, `max_size`: Some(20), added: 2495, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:2 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `MultiTransactionPayment::AccountCurrencyMap` (r:1 w:0)
	/// Proof: `MultiTransactionPayment::AccountCurrencyMap` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	/// Storage: `MultiTransactionPayment::AcceptedCurrencies` (r:1 w:0)
	/// Proof: `MultiTransactionPayment::AcceptedCurrencies` (`max_values`: None, `max_size`: Some(28), added: 2503, mode: `MaxEncodedLen`)
	/// Storage: `EmaOracle::Accumulator` (r:1 w:1)
	/// Proof: `EmaOracle::Accumulator` (`max_values`: Some(1), `max_size`: Some(5921), added: 6416, mode: `MaxEncodedLen`)
	fn withdraw_asset_amount() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3231`
		//  Estimated: `19071`
		// Minimum execution time: 1_061_520_000 picoseconds.
		Weight::from_parts(1_068_689_000, 19071)
			.saturating_add(T::DbWeight::get().reads(23))
			.saturating_add(T::DbWeight::get().writes(6))
	}
	/// Storage: `Stableswap::AssetTradability` (r:2 w:0)
	/// Proof: `Stableswap::AssetTradability` (`max_values`: None, `max_size`: Some(41), added: 2516, mode: `MaxEncodedLen`)
	/// Storage: `Tokens::Accounts` (r:7 w:4)
	/// Proof: `Tokens::Accounts` (`max_values`: None, `max_size`: Some(108), added: 2583, mode: `MaxEncodedLen`)
	/// Storage: `Stableswap::Pools` (r:1 w:0)
	/// Proof: `Stableswap::Pools` (`max_values`: None, `max_size`: Some(57), added: 2532, mode: `MaxEncodedLen`)
	/// Storage: `AssetRegistry::Assets` (r:5 w:0)
	/// Proof: `AssetRegistry::Assets` (`max_values`: None, `max_size`: Some(125), added: 2600, mode: `MaxEncodedLen`)
	/// Storage: `Duster::AccountBlacklist` (r:2 w:0)
	/// Proof: `Duster::AccountBlacklist` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `AssetRegistry::BannedAssets` (r:2 w:0)
	/// Proof: `AssetRegistry::BannedAssets` (`max_values`: None, `max_size`: Some(20), added: 2495, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:2 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `MultiTransactionPayment::AcceptedCurrencies` (r:1 w:0)
	/// Proof: `MultiTransactionPayment::AcceptedCurrencies` (`max_values`: None, `max_size`: Some(28), added: 2503, mode: `MaxEncodedLen`)
	/// Storage: `Tokens::TotalIssuance` (r:1 w:0)
	/// Proof: `Tokens::TotalIssuance` (`max_values`: None, `max_size`: Some(28), added: 2503, mode: `MaxEncodedLen`)
	/// Storage: `EmaOracle::Accumulator` (r:1 w:1)
	/// Proof: `EmaOracle::Accumulator` (`max_values`: Some(1), `max_size`: Some(5921), added: 6416, mode: `MaxEncodedLen`)
	/// Storage: `MultiTransactionPayment::AccountCurrencyMap` (r:0 w:1)
	/// Proof: `MultiTransactionPayment::AccountCurrencyMap` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	fn sell() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3269`
		//  Estimated: `19071`
		// Minimum execution time: 767_301_000 picoseconds.
		Weight::from_parts(771_963_000, 19071)
			.saturating_add(T::DbWeight::get().reads(24))
			.saturating_add(T::DbWeight::get().writes(7))
	}
	/// Storage: `Stableswap::AssetTradability` (r:2 w:0)
	/// Proof: `Stableswap::AssetTradability` (`max_values`: None, `max_size`: Some(41), added: 2516, mode: `MaxEncodedLen`)
	/// Storage: `Stableswap::Pools` (r:1 w:0)
	/// Proof: `Stableswap::Pools` (`max_values`: None, `max_size`: Some(57), added: 2532, mode: `MaxEncodedLen`)
	/// Storage: `Tokens::Accounts` (r:7 w:4)
	/// Proof: `Tokens::Accounts` (`max_values`: None, `max_size`: Some(108), added: 2583, mode: `MaxEncodedLen`)
	/// Storage: `AssetRegistry::Assets` (r:5 w:0)
	/// Proof: `AssetRegistry::Assets` (`max_values`: None, `max_size`: Some(125), added: 2600, mode: `MaxEncodedLen`)
	/// Storage: `Duster::AccountBlacklist` (r:2 w:0)
	/// Proof: `Duster::AccountBlacklist` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `AssetRegistry::BannedAssets` (r:2 w:0)
	/// Proof: `AssetRegistry::BannedAssets` (`max_values`: None, `max_size`: Some(20), added: 2495, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:2 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `MultiTransactionPayment::AccountCurrencyMap` (r:1 w:0)
	/// Proof: `MultiTransactionPayment::AccountCurrencyMap` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	/// Storage: `MultiTransactionPayment::AcceptedCurrencies` (r:1 w:0)
	/// Proof: `MultiTransactionPayment::AcceptedCurrencies` (`max_values`: None, `max_size`: Some(28), added: 2503, mode: `MaxEncodedLen`)
	/// Storage: `Tokens::TotalIssuance` (r:1 w:0)
	/// Proof: `Tokens::TotalIssuance` (`max_values`: None, `max_size`: Some(28), added: 2503, mode: `MaxEncodedLen`)
	/// Storage: `EmaOracle::Accumulator` (r:1 w:1)
	/// Proof: `EmaOracle::Accumulator` (`max_values`: Some(1), `max_size`: Some(5921), added: 6416, mode: `MaxEncodedLen`)
	fn buy() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3239`
		//  Estimated: `19071`
		// Minimum execution time: 745_716_000 picoseconds.
		Weight::from_parts(748_811_000, 19071)
			.saturating_add(T::DbWeight::get().reads(25))
			.saturating_add(T::DbWeight::get().writes(6))
	}
	/// Storage: `Stableswap::Pools` (r:1 w:0)
	/// Proof: `Stableswap::Pools` (`max_values`: None, `max_size`: Some(57), added: 2532, mode: `MaxEncodedLen`)
	/// Storage: `Stableswap::AssetTradability` (r:1 w:1)
	/// Proof: `Stableswap::AssetTradability` (`max_values`: None, `max_size`: Some(41), added: 2516, mode: `MaxEncodedLen`)
	fn set_asset_tradable_state() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `291`
		//  Estimated: `3522`
		// Minimum execution time: 25_044_000 picoseconds.
		Weight::from_parts(25_584_000, 3522)
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Stableswap::Pools` (r:1 w:1)
	/// Proof: `Stableswap::Pools` (`max_values`: None, `max_size`: Some(57), added: 2532, mode: `MaxEncodedLen`)
	fn update_pool_fee() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `291`
		//  Estimated: `3522`
		// Minimum execution time: 22_703_000 picoseconds.
		Weight::from_parts(23_093_000, 3522)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Stableswap::Pools` (r:1 w:1)
	/// Proof: `Stableswap::Pools` (`max_values`: None, `max_size`: Some(57), added: 2532, mode: `MaxEncodedLen`)
	fn update_amplification() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `291`
		//  Estimated: `3522`
		// Minimum execution time: 24_632_000 picoseconds.
		Weight::from_parts(24_919_000, 3522)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Stableswap::Pools` (r:1 w:0)
	/// Proof: `Stableswap::Pools` (`max_values`: None, `max_size`: Some(57), added: 2532, mode: `MaxEncodedLen`)
	/// Storage: `Tokens::Accounts` (r:7 w:4)
	/// Proof: `Tokens::Accounts` (`max_values`: None, `max_size`: Some(108), added: 2583, mode: `MaxEncodedLen`)
	/// Storage: `AssetRegistry::Assets` (r:5 w:0)
	/// Proof: `AssetRegistry::Assets` (`max_values`: None, `max_size`: Some(125), added: 2600, mode: `MaxEncodedLen`)
	/// Storage: `Stableswap::AssetTradability` (r:2 w:0)
	/// Proof: `Stableswap::AssetTradability` (`max_values`: None, `max_size`: Some(41), added: 2516, mode: `MaxEncodedLen`)
	/// Storage: `Duster::AccountBlacklist` (r:2 w:0)
	/// Proof: `Duster::AccountBlacklist` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `AssetRegistry::BannedAssets` (r:2 w:0)
	/// Proof: `AssetRegistry::BannedAssets` (`max_values`: None, `max_size`: Some(20), added: 2495, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:2 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `MultiTransactionPayment::AcceptedCurrencies` (r:1 w:0)
	/// Proof: `MultiTransactionPayment::AcceptedCurrencies` (`max_values`: None, `max_size`: Some(28), added: 2503, mode: `MaxEncodedLen`)
	/// Storage: `Tokens::TotalIssuance` (r:1 w:0)
	/// Proof: `Tokens::TotalIssuance` (`max_values`: None, `max_size`: Some(28), added: 2503, mode: `MaxEncodedLen`)
	/// Storage: `EmaOracle::Accumulator` (r:1 w:1)
	/// Proof: `EmaOracle::Accumulator` (`max_values`: Some(1), `max_size`: Some(5921), added: 6416, mode: `MaxEncodedLen`)
	/// Storage: `MultiTransactionPayment::AccountCurrencyMap` (r:0 w:1)
	/// Proof: `MultiTransactionPayment::AccountCurrencyMap` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	/// The range of component `c` is `[1, 2]`.
	/// The range of component `e` is `[0, 1]`.
	fn router_execution_sell(_c: u32, e: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1555 + e * (1714 ±0)`
		//  Estimated: `13990 + e * (5797 ±245_709_589_663_843_264)`
		// Minimum execution time: 314_654_000 picoseconds.
		Weight::from_parts(321_208_138, 13990)
			// Standard Error: 305_814
			.saturating_add(Weight::from_parts(731_985_555, 0).saturating_mul(e.into()))
			.saturating_add(T::DbWeight::get().reads(11))
			.saturating_add(T::DbWeight::get().reads((13_u64).saturating_mul(e.into())))
			.saturating_add(T::DbWeight::get().writes((7_u64).saturating_mul(e.into())))
			.saturating_add(Weight::from_parts(0, 5797).saturating_mul(e.into()))
	}
	/// Storage: `Stableswap::Pools` (r:1 w:0)
	/// Proof: `Stableswap::Pools` (`max_values`: None, `max_size`: Some(57), added: 2532, mode: `MaxEncodedLen`)
	/// Storage: `Tokens::Accounts` (r:7 w:4)
	/// Proof: `Tokens::Accounts` (`max_values`: None, `max_size`: Some(108), added: 2583, mode: `MaxEncodedLen`)
	/// Storage: `AssetRegistry::Assets` (r:5 w:0)
	/// Proof: `AssetRegistry::Assets` (`max_values`: None, `max_size`: Some(125), added: 2600, mode: `MaxEncodedLen`)
	/// Storage: `Stableswap::AssetTradability` (r:2 w:0)
	/// Proof: `Stableswap::AssetTradability` (`max_values`: None, `max_size`: Some(41), added: 2516, mode: `MaxEncodedLen`)
	/// Storage: `Duster::AccountBlacklist` (r:2 w:0)
	/// Proof: `Duster::AccountBlacklist` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `AssetRegistry::BannedAssets` (r:2 w:0)
	/// Proof: `AssetRegistry::BannedAssets` (`max_values`: None, `max_size`: Some(20), added: 2495, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:2 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `MultiTransactionPayment::AccountCurrencyMap` (r:1 w:0)
	/// Proof: `MultiTransactionPayment::AccountCurrencyMap` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	/// Storage: `MultiTransactionPayment::AcceptedCurrencies` (r:1 w:0)
	/// Proof: `MultiTransactionPayment::AcceptedCurrencies` (`max_values`: None, `max_size`: Some(28), added: 2503, mode: `MaxEncodedLen`)
	/// Storage: `Tokens::TotalIssuance` (r:1 w:0)
	/// Proof: `Tokens::TotalIssuance` (`max_values`: None, `max_size`: Some(28), added: 2503, mode: `MaxEncodedLen`)
	/// Storage: `EmaOracle::Accumulator` (r:1 w:1)
	/// Proof: `EmaOracle::Accumulator` (`max_values`: Some(1), `max_size`: Some(5921), added: 6416, mode: `MaxEncodedLen`)
	/// The range of component `c` is `[1, 2]`.
	/// The range of component `e` is `[0, 1]`.
	fn router_execution_buy(c: u32, e: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1554 + e * (1685 ±0)`
		//  Estimated: `13990 + e * (5797 ±201_913_854_157_949_888)`
		// Minimum execution time: 312_997_000 picoseconds.
		Weight::from_parts(314_359_000, 13990)
			// Standard Error: 3_248_945
			.saturating_add(Weight::from_parts(12_345_948, 0).saturating_mul(c.into()))
			// Standard Error: 7_132_362
			.saturating_add(Weight::from_parts(468_057_036, 0).saturating_mul(e.into()))
			.saturating_add(T::DbWeight::get().reads(11))
			.saturating_add(T::DbWeight::get().reads((14_u64).saturating_mul(e.into())))
			.saturating_add(T::DbWeight::get().writes((6_u64).saturating_mul(e.into())))
			.saturating_add(Weight::from_parts(0, 5797).saturating_mul(e.into()))
	}
}